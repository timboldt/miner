import('./pkg')
.then(rust_module => {
  let game = new rust_module.WebGame();
  for (i = 0; i < 100; i++) {
    game.move_player("r");
  }
  document.getElementById("txtout").innerText = game.get_text_repr();
})
.catch(console.error);