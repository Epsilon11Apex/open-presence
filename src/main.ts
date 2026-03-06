import { invoke } from "@tauri-apps/api/core";

// Cette fonction s'exécute dès que la fenêtre s'affiche
window.addEventListener("DOMContentLoaded", async () => {
  console.log("Fenêtre prête, tentative de connexion Discord...");
  
  try {
    // On appelle la fonction Rust "update_status"
    await invoke("update_status", { 
      details: "Création d'une alternative Premid", 
      label: "Phase de test" 
    });
    console.log("Succès : Les infos ont été envoyées au Bridge Rust !");
  } catch (err) {
    console.error("Erreur lors de l'appel au Bridge :", err);
  }
});