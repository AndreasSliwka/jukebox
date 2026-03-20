function filterTable(raw_term) {
  let term = raw_term.toLowerCase();

  document.cookie =
    "search=" + term.replaceAll("'", "\\'") + ";SameSite=strict";

  let table = document.getElementById("songlist");
  let song_trs = table.getElementsByTagName("tr");
  for (const song_tr of song_trs) {
    if (song_tr.hasAttribute("data-name")) {
      const title = song_tr.getAttribute("data-name");
      if (title.includes(term)) {
        song_tr.classList.remove("hidden");
      } else {
        song_tr.classList.add("hidden");
      }
    }
  }
}

function getCookieValue(cookieName) {
  let match = document.cookie.match(new RegExp(cookieName + "=([^;]*)(;|$)"));
  let value;
  if (match) {
    value = match[1];
  } else {
    value = "";
  }
  console.log("  cookie = " + cookieName + " = " + value);
  return value;
}

function setCookieValue(cookieName, value) {
  document.cookie = cookieName + "=" + value + ";SameSite=strict";
  console.log("document.cookie=" + document.cookie);
}

function maybeApplyShowChords() {
  let song = window.document.getElementById("song");
  if (song) {
    if (getCookieValue("showChords") == "true") {
      song.classList.add("showChords");
    } else {
      song.classList.remove("showChords");
    }
  }
}
