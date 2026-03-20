function filterTable(raw_term) {
  let term = raw_term.toLowerCase();

  document.cookie =
    "search=" + term.replaceAll("'", "\\'") + ";SameSite=strict";

  let table = document.getElementById("songlist");
  let song_trs = table.getElementsByTagName("tr");
  for (const song_tr of song_trs) {
    if (song_tr.hasAttribute("data-name")) {
      const title = song_tr.getAttribute("data-name");
      console.log("title = " + title);
      if (title.includes(term)) {
        song_tr.classList.remove("hidden");
      } else {
        song_tr.classList.add("hidden");
      }
    }
  }
}
