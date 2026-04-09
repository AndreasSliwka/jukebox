function filter_song_list(field, term) {
  const song_trs = document
    .getElementById("songlist")
    .getElementsByTagName("tr");
  for (const song_tr of song_trs) {
    if (song_tr.hasAttribute(field)) {
      const data = song_tr.getAttribute(field);
      if (data.includes(term)) {
        song_tr.classList.remove("hidden");
      } else {
        song_tr.classList.add("hidden");
      }
    }
  }
}
function filterTableByText(raw_term) {
  let term = raw_term.toLowerCase();

  document.cookie = "search=" + term.replaceAll("'", "\\'") + ";SameSite=lax";
  document.cookie = "category=;SameSite=lax";
  deselect_category_wof_entries();

  filter_song_list("data-name", raw_term.toLowerCase());
}

function getCookieValue(cookieName) {
  let match = document.cookie.match(new RegExp(cookieName + "=([^;]*)(;|$)"));
  let value;
  if (match) {
    value = match[1];
  } else {
    value = "";
  }
  return value;
}

function setCookieValue(cookieName, value) {
  document.cookie = cookieName + "=" + value + ";SameSite=lax";
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

function maybeApplyHidePlayedSongs() {
  let songlist = window.document.getElementById("songlist");
  if (songlist) {
    if (getCookieValue("hidePlayedSongs") == "true") {
      songlist.classList.add("hidePlayedSongs");
    } else {
      songlist.classList.remove("hidePlayedSongs");
    }
  }
}

function setSearchFilter(term) {
  if (term.startsWith("admin:")) {
    let passkey = term.replace(/^admin:/, "");
    let endpoint = window.location.origin + "/admin?passkey=" + passkey;
    window.location.href = endpoint;
  } else {
    filterTableByText(term);
    window.document.getElementById("song_search").scrollIntoView(false);
  }
}

function deselect_category_wof_entries() {
  Array.from(
    document
      .getElementById("wheel_of_fortune")
      .getElementsByClassName("category selected"),
  ).forEach((category) => {
    category.classList.remove("selected");
  });
}

document.addEventListener("alpine:init", () => {
  Alpine.store("wof", {
    visible_categories: [],
    categories_by_name: {},

    initialize(categories_by_name) {
      this.categories_by_name = categories_by_name;
    },
    reshuffle() {
      var keys = Object.keys(this.categories_by_name);
      var randomized = keys.sort(() => Math.random() - 0.5);
      var sliced = randomized.slice(0, 3);
      this.visible_categories = [
        { name: sliced[0], sign: this.categories_by_name[sliced[0]] },
        { name: sliced[1], sign: this.categories_by_name[sliced[1]] },
        { name: sliced[2], sign: this.categories_by_name[sliced[2]] },
      ];
    },
    filterListByCategory(target) {
      // set category cookie, clear search cookie and input
      document.cookie = "category=" + target.textContent + ";SameSite=lax";
      document.cookie = "search=;SameSite=lax";
      document.getElementById("song_search").value = "";

      // deselect all category <spans>
      deselect_category_wof_entries();

      // select current category <span>
      target.parentElement.classList.add("selected");

      filter_song_list("data-categories", "|" + target.textContent + "|");
    },
  });
});

function setup_wof(categories_by_name) {
  Alpine.store("wof").initialize(categories_by_name);
  Alpine.store("wof").reshuffle();
}

function changeZoom(offset) {
  let main = document.getElementById("root_of_all_evil");
  let current_zoom = main.className;
  let zoom_level = parseInt(current_zoom.split("-")[1]);
  let new_zoom = zoom_level + offset;
  if (new_zoom < 0) {
    new_zoom = 0;
  } else if (new_zoom > 7) {
    new_zoom = 7;
  }
  main.className = "zoom-" + new_zoom;
  document.cookie = "zoom=" + new_zoom + ";SameSite=lax";
}
