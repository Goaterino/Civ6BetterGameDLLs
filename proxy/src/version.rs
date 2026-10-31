//! Version de BetterGameDLLs, annoncée aux autres joueurs dans le salon multijoueur par le proxy de
//! démarrage (`startup_proxy/src/salon.rs`). Les correctifs POL (« proxy-only lobbies »), ceux qui
//! changent la simulation, ne s'activeront que si tous les humains de la partie annoncent exactement
//! la même : l'augmenter à chaque version qui change la simulation ou l'échange du salon.

pub const VERSION: &str = "0.1.0";
