use libc;

pub const MESGSIZE: usize = 256; /* max #bytes per message, incl. null at end */
pub const NMESG: usize = 16; /* max #messages */

#[repr(C)]
pub struct shmstruct {
  pub mutex: libc::sem_t,
  pub nempty: libc::sem_t,
  pub nstored: libc::sem_t,
  pub nput: libc::c_int,
  pub noverflow: libc::c_long,
  pub noverflowmutex: libc::sem_t,
  pub msgoff: [libc::c_long; NMESG],
  pub msgdata: [libc::c_char; NMESG * MESGSIZE],
}
