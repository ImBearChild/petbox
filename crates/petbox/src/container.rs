use std::{
    fs::{self, DirBuilder},
    io::{self, BufRead},
    path::Path,
};
use unshare_petbox::{GidMap, Namespace, UidMap};

#[derive(Debug)]
struct SubXidMap {
    xid: u32,
    subxid: u32,
    count: u32,
}

fn get_subxid(path: &Path) -> Vec<SubXidMap> {
    let mut mapvec = Vec::new();
    let t = fs::File::open(path).unwrap();
    for i in io::BufReader::new(t).lines() {
        let mut map = SubXidMap {
            xid: 0,
            subxid: 0,
            count: 0,
        };
        let i = i.unwrap();
        trace!("{:?}", &i);
        let v: Vec<&str> = i.split(":").collect();

        let u = v[0].parse::<u32>();
        map.xid = match u {
            Ok(u) => u,
            Err(_) => nix::unistd::User::from_name(v[0])
                .unwrap()
                .unwrap()
                .uid
                .into(),
        };

        map.subxid = v[1].parse::<u32>().unwrap();
        map.count = v[2].parse::<u32>().unwrap();
        trace!("{:#?}", map);
        mapvec.push(map);
    }
    mapvec
}

fn get_current_subuidmap() -> SubXidMap {
    let v = get_subxid(Path::new("/etc/subuid"));
    let mut m: Option<SubXidMap> = None;
    for i in v {
        if i.xid == nix::unistd::getuid().into() {
            m = Some(i);
        }
    }
    m.unwrap()
}

fn gen_uidmap() -> Vec<UidMap> {
    let mut mapvec = Vec::new();
    let subuidmap = get_current_subuidmap();
    trace!("subuidmap for current user:");
    trace!("{:#?}", subuidmap);
    mapvec.push(UidMap {
        inside_uid: subuidmap.xid,
        outside_uid: subuidmap.xid,
        count: 1,
    }); // Map for current user inside the container
    mapvec.push(UidMap {
        inside_uid: 0,
        outside_uid: subuidmap.subxid,
        count: 1000,
    });
    mapvec
}

fn gen_gidmap() -> Vec<GidMap> {
    let mut mapvec = Vec::new();
    let subuidmap = get_current_subuidmap();
    trace!("subuidmap for current user:");
    trace!("{:#?}", subuidmap);
    mapvec.push(GidMap {
        inside_gid: subuidmap.xid,
        outside_gid: subuidmap.xid,
        count: 1,
    }); // Map for current user inside the container
    mapvec.push(GidMap {
        inside_gid: 0,
        outside_gid: subuidmap.subxid,
        count: 1000,
    });
    mapvec
}

pub fn install_rootfs(path: &Path, tar_file: &Path, dry_run: bool, enter_ns: bool) {
    info!("Creating conatiner...");
    trace!("path:{:?},tar_file:{:?}", path, tar_file);

    DirBuilder::new().recursive(true).create(path).unwrap();

    let uid_map = gen_uidmap();
    trace!("uidmap:");
    trace!("{:#?}", uid_map);
    let gid_map = gen_gidmap();
    let mut cmd:unshare_petbox::Command;
    match enter_ns {
        true => { cmd = unshare_petbox::Command::new("/usr/bin/bash");},
        false => {
            cmd = unshare_petbox::Command::new("/usr/bin/tar");
            cmd.arg("xf")
                .arg(tar_file.as_os_str())
                .arg("--directory")
                .arg(path.as_os_str());
        }
    }
    let mut namespaces = Vec::<Namespace>::new();
    namespaces.push(Namespace::User);
    cmd.set_id_maps(uid_map, gid_map)
        .set_id_map_commands("/usr/bin/newuidmap", "/usr/bin/newgidmap");
    cmd.unshare(&namespaces);

    cmd.uid(0);
    cmd.gid(0);
    if dry_run {
        info!("Dry run mode. No changes have been applied.");
        return;
    }
    info!("");
    match cmd.status().unwrap() {
        // propagate signal
        unshare_petbox::ExitStatus::Exited(x) => println!("{}", x),
        unshare_petbox::ExitStatus::Signaled(x, _) => println!("{}", x),
    }
}
