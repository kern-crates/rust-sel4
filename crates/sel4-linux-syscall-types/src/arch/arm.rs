// https://git.musl-libc.org/cgit/musl/tree/arch/arm/bits/syscall.h.in

pub mod NR {
    pub const restart_syscall: isize = 0;
    pub const exit: isize = 1;
    pub const fork: isize = 2;
    pub const read: isize = 3;
    pub const write: isize = 4;
    pub const open: isize = 5;
    pub const close: isize = 6;
    pub const creat: isize = 8;
    pub const link: isize = 9;
    pub const unlink: isize = 10;
    pub const execve: isize = 11;
    pub const chdir: isize = 12;
    pub const mknod: isize = 14;
    pub const chmod: isize = 15;
    pub const lchown: isize = 16;
    pub const lseek: isize = 19;
    pub const getpid: isize = 20;
    pub const mount: isize = 21;
    pub const setuid: isize = 23;
    pub const getuid: isize = 24;
    pub const ptrace: isize = 26;
    pub const pause: isize = 29;
    pub const access: isize = 33;
    pub const nice: isize = 34;
    pub const sync: isize = 36;
    pub const kill: isize = 37;
    pub const rename: isize = 38;
    pub const mkdir: isize = 39;
    pub const rmdir: isize = 40;
    pub const dup: isize = 41;
    pub const pipe: isize = 42;
    pub const times: isize = 43;
    pub const brk: isize = 45;
    pub const setgid: isize = 46;
    pub const getgid: isize = 47;
    pub const geteuid: isize = 49;
    pub const getegid: isize = 50;
    pub const acct: isize = 51;
    pub const umount2: isize = 52;
    pub const ioctl: isize = 54;
    pub const fcntl: isize = 55;
    pub const setpgid: isize = 57;
    pub const umask: isize = 60;
    pub const chroot: isize = 61;
    pub const ustat: isize = 62;
    pub const dup2: isize = 63;
    pub const getppid: isize = 64;
    pub const getpgrp: isize = 65;
    pub const setsid: isize = 66;
    pub const sigaction: isize = 67;
    pub const setreuid: isize = 70;
    pub const setregid: isize = 71;
    pub const sigsuspend: isize = 72;
    pub const sigpending: isize = 73;
    pub const sethostname: isize = 74;
    pub const setrlimit: isize = 75;
    pub const getrusage: isize = 77;
    pub const gettimeofday_time32: isize = 78;
    pub const settimeofday_time32: isize = 79;
    pub const getgroups: isize = 80;
    pub const setgroups: isize = 81;
    pub const symlink: isize = 83;
    pub const readlink: isize = 85;
    pub const uselib: isize = 86;
    pub const swapon: isize = 87;
    pub const reboot: isize = 88;
    pub const munmap: isize = 91;
    pub const truncate: isize = 92;
    pub const ftruncate: isize = 93;
    pub const fchmod: isize = 94;
    pub const fchown: isize = 95;
    pub const getpriority: isize = 96;
    pub const setpriority: isize = 97;
    pub const statfs: isize = 99;
    pub const fstatfs: isize = 100;
    pub const syslog: isize = 103;
    pub const setitimer: isize = 104;
    pub const getitimer: isize = 105;
    pub const stat: isize = 106;
    pub const lstat: isize = 107;
    pub const fstat: isize = 108;
    pub const vhangup: isize = 111;
    pub const wait4: isize = 114;
    pub const swapoff: isize = 115;
    pub const sysinfo: isize = 116;
    pub const fsync: isize = 118;
    pub const sigreturn: isize = 119;
    pub const clone: isize = 120;
    pub const setdomainname: isize = 121;
    pub const uname: isize = 122;
    pub const adjtimex: isize = 124;
    pub const mprotect: isize = 125;
    pub const sigprocmask: isize = 126;
    pub const init_module: isize = 128;
    pub const delete_module: isize = 129;
    pub const quotactl: isize = 131;
    pub const getpgid: isize = 132;
    pub const fchdir: isize = 133;
    pub const bdflush: isize = 134;
    pub const sysfs: isize = 135;
    pub const personality: isize = 136;
    pub const setfsuid: isize = 138;
    pub const setfsgid: isize = 139;
    pub const _llseek: isize = 140;
    pub const getdents: isize = 141;
    pub const _newselect: isize = 142;
    pub const flock: isize = 143;
    pub const msync: isize = 144;
    pub const readv: isize = 145;
    pub const writev: isize = 146;
    pub const getsid: isize = 147;
    pub const fdatasync: isize = 148;
    pub const _sysctl: isize = 149;
    pub const mlock: isize = 150;
    pub const munlock: isize = 151;
    pub const mlockall: isize = 152;
    pub const munlockall: isize = 153;
    pub const sched_setparam: isize = 154;
    pub const sched_getparam: isize = 155;
    pub const sched_setscheduler: isize = 156;
    pub const sched_getscheduler: isize = 157;
    pub const sched_yield: isize = 158;
    pub const sched_get_priority_max: isize = 159;
    pub const sched_get_priority_min: isize = 160;
    pub const sched_rr_get_interval: isize = 161;
    pub const nanosleep: isize = 162;
    pub const mremap: isize = 163;
    pub const setresuid: isize = 164;
    pub const getresuid: isize = 165;
    pub const poll: isize = 168;
    pub const nfsservctl: isize = 169;
    pub const setresgid: isize = 170;
    pub const getresgid: isize = 171;
    pub const prctl: isize = 172;
    pub const rt_sigreturn: isize = 173;
    pub const rt_sigaction: isize = 174;
    pub const rt_sigprocmask: isize = 175;
    pub const rt_sigpending: isize = 176;
    pub const rt_sigtimedwait: isize = 177;
    pub const rt_sigqueueinfo: isize = 178;
    pub const rt_sigsuspend: isize = 179;
    pub const pread64: isize = 180;
    pub const pwrite64: isize = 181;
    pub const chown: isize = 182;
    pub const getcwd: isize = 183;
    pub const capget: isize = 184;
    pub const capset: isize = 185;
    pub const sigaltstack: isize = 186;
    pub const sendfile: isize = 187;
    pub const vfork: isize = 190;
    pub const ugetrlimit: isize = 191;
    pub const mmap2: isize = 192;
    pub const truncate64: isize = 193;
    pub const ftruncate64: isize = 194;
    pub const stat64: isize = 195;
    pub const lstat64: isize = 196;
    pub const fstat64: isize = 197;
    pub const lchown32: isize = 198;
    pub const getuid32: isize = 199;
    pub const getgid32: isize = 200;
    pub const geteuid32: isize = 201;
    pub const getegid32: isize = 202;
    pub const setreuid32: isize = 203;
    pub const setregid32: isize = 204;
    pub const getgroups32: isize = 205;
    pub const setgroups32: isize = 206;
    pub const fchown32: isize = 207;
    pub const setresuid32: isize = 208;
    pub const getresuid32: isize = 209;
    pub const setresgid32: isize = 210;
    pub const getresgid32: isize = 211;
    pub const chown32: isize = 212;
    pub const setuid32: isize = 213;
    pub const setgid32: isize = 214;
    pub const setfsuid32: isize = 215;
    pub const setfsgid32: isize = 216;
    pub const getdents64: isize = 217;
    pub const pivot_root: isize = 218;
    pub const mincore: isize = 219;
    pub const madvise: isize = 220;
    pub const fcntl64: isize = 221;
    pub const gettid: isize = 224;
    pub const readahead: isize = 225;
    pub const setxattr: isize = 226;
    pub const lsetxattr: isize = 227;
    pub const fsetxattr: isize = 228;
    pub const getxattr: isize = 229;
    pub const lgetxattr: isize = 230;
    pub const fgetxattr: isize = 231;
    pub const listxattr: isize = 232;
    pub const llistxattr: isize = 233;
    pub const flistxattr: isize = 234;
    pub const removexattr: isize = 235;
    pub const lremovexattr: isize = 236;
    pub const fremovexattr: isize = 237;
    pub const tkill: isize = 238;
    pub const sendfile64: isize = 239;
    pub const futex: isize = 240;
    pub const sched_setaffinity: isize = 241;
    pub const sched_getaffinity: isize = 242;
    pub const io_setup: isize = 243;
    pub const io_destroy: isize = 244;
    pub const io_getevents: isize = 245;
    pub const io_submit: isize = 246;
    pub const io_cancel: isize = 247;
    pub const exit_group: isize = 248;
    pub const lookup_dcookie: isize = 249;
    pub const epoll_create: isize = 250;
    pub const epoll_ctl: isize = 251;
    pub const epoll_wait: isize = 252;
    pub const remap_file_pages: isize = 253;
    pub const set_tid_address: isize = 256;
    pub const timer_create: isize = 257;
    pub const timer_settime32: isize = 258;
    pub const timer_gettime32: isize = 259;
    pub const timer_getoverrun: isize = 260;
    pub const timer_delete: isize = 261;
    pub const clock_settime32: isize = 262;
    pub const clock_gettime32: isize = 263;
    pub const clock_getres_time32: isize = 264;
    pub const clock_nanosleep_time32: isize = 265;
    pub const statfs64: isize = 266;
    pub const fstatfs64: isize = 267;
    pub const tgkill: isize = 268;
    pub const utimes: isize = 269;
    pub const fadvise64_64: isize = 270;
    pub const arm_fadvise64_64: isize = 270;
    pub const pciconfig_iobase: isize = 271;
    pub const pciconfig_read: isize = 272;
    pub const pciconfig_write: isize = 273;
    pub const mq_open: isize = 274;
    pub const mq_unlink: isize = 275;
    pub const mq_timedsend: isize = 276;
    pub const mq_timedreceive: isize = 277;
    pub const mq_notify: isize = 278;
    pub const mq_getsetattr: isize = 279;
    pub const waitid: isize = 280;
    pub const socket: isize = 281;
    pub const bind: isize = 282;
    pub const connect: isize = 283;
    pub const listen: isize = 284;
    pub const accept: isize = 285;
    pub const getsockname: isize = 286;
    pub const getpeername: isize = 287;
    pub const socketpair: isize = 288;
    pub const send: isize = 289;
    pub const sendto: isize = 290;
    pub const recv: isize = 291;
    pub const recvfrom: isize = 292;
    pub const shutdown: isize = 293;
    pub const setsockopt: isize = 294;
    pub const getsockopt: isize = 295;
    pub const sendmsg: isize = 296;
    pub const recvmsg: isize = 297;
    pub const semop: isize = 298;
    pub const semget: isize = 299;
    pub const semctl: isize = 300;
    pub const msgsnd: isize = 301;
    pub const msgrcv: isize = 302;
    pub const msgget: isize = 303;
    pub const msgctl: isize = 304;
    pub const shmat: isize = 305;
    pub const shmdt: isize = 306;
    pub const shmget: isize = 307;
    pub const shmctl: isize = 308;
    pub const add_key: isize = 309;
    pub const request_key: isize = 310;
    pub const keyctl: isize = 311;
    pub const semtimedop: isize = 312;
    pub const vserver: isize = 313;
    pub const ioprio_set: isize = 314;
    pub const ioprio_get: isize = 315;
    pub const inotify_init: isize = 316;
    pub const inotify_add_watch: isize = 317;
    pub const inotify_rm_watch: isize = 318;
    pub const mbind: isize = 319;
    pub const get_mempolicy: isize = 320;
    pub const set_mempolicy: isize = 321;
    pub const openat: isize = 322;
    pub const mkdirat: isize = 323;
    pub const mknodat: isize = 324;
    pub const fchownat: isize = 325;
    pub const futimesat: isize = 326;
    pub const fstatat64: isize = 327;
    pub const unlinkat: isize = 328;
    pub const renameat: isize = 329;
    pub const linkat: isize = 330;
    pub const symlinkat: isize = 331;
    pub const readlinkat: isize = 332;
    pub const fchmodat: isize = 333;
    pub const faccessat: isize = 334;
    pub const pselect6: isize = 335;
    pub const ppoll: isize = 336;
    pub const unshare: isize = 337;
    pub const set_robust_list: isize = 338;
    pub const get_robust_list: isize = 339;
    pub const splice: isize = 340;
    pub const sync_file_range2: isize = 341;
    pub const arm_sync_file_range: isize = 341;
    pub const tee: isize = 342;
    pub const vmsplice: isize = 343;
    pub const move_pages: isize = 344;
    pub const getcpu: isize = 345;
    pub const epoll_pwait: isize = 346;
    pub const kexec_load: isize = 347;
    pub const utimensat: isize = 348;
    pub const signalfd: isize = 349;
    pub const timerfd_create: isize = 350;
    pub const eventfd: isize = 351;
    pub const fallocate: isize = 352;
    pub const timerfd_settime32: isize = 353;
    pub const timerfd_gettime32: isize = 354;
    pub const signalfd4: isize = 355;
    pub const eventfd2: isize = 356;
    pub const epoll_create1: isize = 357;
    pub const dup3: isize = 358;
    pub const pipe2: isize = 359;
    pub const inotify_init1: isize = 360;
    pub const preadv: isize = 361;
    pub const pwritev: isize = 362;
    pub const rt_tgsigqueueinfo: isize = 363;
    pub const perf_event_open: isize = 364;
    pub const recvmmsg: isize = 365;
    pub const accept4: isize = 366;
    pub const fanotify_init: isize = 367;
    pub const fanotify_mark: isize = 368;
    pub const prlimit64: isize = 369;
    pub const name_to_handle_at: isize = 370;
    pub const open_by_handle_at: isize = 371;
    pub const clock_adjtime: isize = 372;
    pub const syncfs: isize = 373;
    pub const sendmmsg: isize = 374;
    pub const setns: isize = 375;
    pub const process_vm_readv: isize = 376;
    pub const process_vm_writev: isize = 377;
    pub const kcmp: isize = 378;
    pub const finit_module: isize = 379;
    pub const sched_setattr: isize = 380;
    pub const sched_getattr: isize = 381;
    pub const renameat2: isize = 382;
    pub const seccomp: isize = 383;
    pub const getrandom: isize = 384;
    pub const memfd_create: isize = 385;
    pub const bpf: isize = 386;
    pub const execveat: isize = 387;
    pub const userfaultfd: isize = 388;
    pub const membarrier: isize = 389;
    pub const mlock2: isize = 390;
    pub const copy_file_range: isize = 391;
    pub const preadv2: isize = 392;
    pub const pwritev2: isize = 393;
    pub const pkey_mprotect: isize = 394;
    pub const pkey_alloc: isize = 395;
    pub const pkey_free: isize = 396;
    pub const statx: isize = 397;
    pub const rseq: isize = 398;
    pub const io_pgetevents: isize = 399;
    pub const migrate_pages: isize = 400;
    pub const kexec_file_load: isize = 401;
    pub const clock_gettime64: isize = 403;
    pub const clock_settime64: isize = 404;
    pub const clock_adjtime64: isize = 405;
    pub const clock_getres_time64: isize = 406;
    pub const clock_nanosleep_time64: isize = 407;
    pub const timer_gettime64: isize = 408;
    pub const timer_settime64: isize = 409;
    pub const timerfd_gettime64: isize = 410;
    pub const timerfd_settime64: isize = 411;
    pub const utimensat_time64: isize = 412;
    pub const pselect6_time64: isize = 413;
    pub const ppoll_time64: isize = 414;
    pub const io_pgetevents_time64: isize = 416;
    pub const recvmmsg_time64: isize = 417;
    pub const mq_timedsend_time64: isize = 418;
    pub const mq_timedreceive_time64: isize = 419;
    pub const semtimedop_time64: isize = 420;
    pub const rt_sigtimedwait_time64: isize = 421;
    pub const futex_time64: isize = 422;
    pub const sched_rr_get_interval_time64: isize = 423;
    pub const pidfd_send_signal: isize = 424;
    pub const io_uring_setup: isize = 425;
    pub const io_uring_enter: isize = 426;
    pub const io_uring_register: isize = 427;
    pub const open_tree: isize = 428;
    pub const move_mount: isize = 429;
    pub const fsopen: isize = 430;
    pub const fsconfig: isize = 431;
    pub const fsmount: isize = 432;
    pub const fspick: isize = 433;
    pub const pidfd_open: isize = 434;
    pub const clone3: isize = 435;
    pub const close_range: isize = 436;
    pub const openat2: isize = 437;
    pub const pidfd_getfd: isize = 438;
    pub const faccessat2: isize = 439;
    pub const process_madvise: isize = 440;
    pub const epoll_pwait2: isize = 441;
    pub const mount_setattr: isize = 442;
    pub const landlock_create_ruleset: isize = 444;
    pub const landlock_add_rule: isize = 445;
    pub const landlock_restrict_self: isize = 446;
    pub const process_mrelease: isize = 448;
    pub const futex_waitv: isize = 449;
    pub const set_mempolicy_home_node: isize = 450;
    pub const cachestat: isize = 451;
    pub const fchmodat2: isize = 452;
}

pub mod ARM_NR {
    pub const breakpoint: isize = 0x0f0001;
    pub const cacheflush: isize = 0x0f0002;
    pub const usr26: isize = 0x0f0003;
    pub const usr32: isize = 0x0f0004;
    pub const set_tls: isize = 0x0f0005;
    pub const get_tls: isize = 0x0f0006;
}
