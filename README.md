PetBox
====

Petbox is a lightweight unprivileged container tool. Its main goal is to provide a configurable container runtime [^container_runtime] for pet containers.

Why PetBox
---

Pets vs. cattle has become a famous analogy for server operations in the infrastructure management space. Container workloads on servers more like cattle, not pets. Operators don't name them, or try hard to fix them when some of them go wrong. They will be killed and restarted, or be reset to some checkpoints. If someone wants to modify its configuration or upgrade software programs in it, he usually kills it and creates a new container.

A pet container is different from conventional "cattle" container. User will name it, take care of it, keep it up to date and modify its configuration when needed.

Usage
---

```bash
petbox create --name void --source ./void-x86_64-ROOTFS-20221001.tar.xz
petbox run --name void -- /sbin/init
petbox exec --name void --tty /bin/sh
```

---

License
---

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at <https://mozilla.org/MPL/2.0/>.

This software is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

[^container_runtime]: The container runtime is the software that is responsible for running containers.
