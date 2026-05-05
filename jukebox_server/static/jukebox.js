SongList = {
  _hide_filtered_out_songs(field, term) {
    const songlist = document.getElementById("songlist");
    if (songlist) {
      const song_trs = songlist.getElementsByClassName("listed-song");
      for (const song of song_trs) {
        if (song.hasAttribute(field)) {
          const data = song.getAttribute(field);
          if (data.includes(term)) {
            song.classList.remove("hidden");
          } else {
            song.classList.add("hidden");
          }
        }
      }
    }
  },
  hide_all_songs() {
    const songlist = document.getElementById("songlist");
    if (songlist) {
      const song_trs = songlist.getElementsByClassName("listed-song");
      for (const song of song_trs) {
        song.classList.add("hidden");
      }
    }
  },
  show_all_songs() {
    const songlist = document.getElementById("songlist");
    if (songlist) {
      const song_trs = songlist.getElementsByClassName("listed-song");
      for (const song of song_trs) {
        song.classList.remove("hidden");
      }
    }
  },
  filterByName(raw_term) {
    let term = raw_term.toLowerCase();

    document.cookie = "search=" + term.replaceAll("'", "\\'") + ";SameSite=lax";
    document.cookie = "category=;SameSite=lax";
    // deselect_category_wof_entries();
    this._hide_filtered_out_songs("data-name", raw_term.toLowerCase());
    if (raw_term == "") {
      Toolbar.hideSearchForm();
    }
  },
  setSearchFilter(input) {
    const term = input.value;
    if (term.startsWith("admin:")) {
      let passkey = term.replace(/^admin:/, "");
      let endpoint = window.location.origin + "/admin?passkey=" + passkey;
      window.location.href = endpoint;
    } else {
      this.filterByName(term);
    }
    input.blur();
  },
  selectSevenRandomSongs(andThen = () => {}) {
    this.show_all_songs();

    const songlist = document.getElementById("songlist");
    songlist.scrollIntoView(true);
    const song_ids = Array.prototype.slice
      .call(songlist.querySelectorAll(".listed-song"))
      .map((el) => el.id);
    selected_ids = song_ids
      .sort(() => 0.5 - Math.random())
      .sort(() => 0.5 - Math.random())
      .sort(() => 0.5 - Math.random())
      .slice(0, 7);
    function drop_next_unselected_id() {
      if (song_ids.length > 0) {
        song_id_to_hide = song_ids.pop();
        if (!selected_ids.includes(song_id_to_hide)) {
          document.getElementById(song_id_to_hide).classList.add("hidden");
        }
        setTimeout(drop_next_unselected_id, 6);
      } else {
        console.log("and then!");
        andThen();
      }
    }

    drop_next_unselected_id();
  },
};
Cookies = {
  getValue(cookieName) {
    let match = document.cookie.match(new RegExp(cookieName + "=([^;]*)(;|$)"));
    let value;
    if (match) {
      value = match[1];
    } else {
      value = "";
    }
    return value;
  },

  setValue(cookieName, value) {
    document.cookie = cookieName + "=" + value + ";SameSite=lax";
  },
  storedShowChords() {
    return this.getValue("showChords") == "true";
  },
};

Chords = {
  showInline() {
    return Cookies.getValue("showChords") == "true";
  },
  toggle() {
    if (this.showInline()) {
      Cookies.setValue("showChords", "false");
    } else {
      Cookies.setValue("showChords", "true");
    }
    this.maybeShow();
  },
  maybeShow() {
    let song = window.document.getElementById("song");
    if (song) {
      if (this.showInline()) {
        song.classList.add("showChords");
      } else {
        song.classList.remove("showChords");
      }
    }
  },
};

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
      //  deselect_category_wof_entries();

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

Zoom = {
  currentZoomFromMainElement() {
    let main = document.getElementById("root_of_all_evil");

    let current_zoom = main.className
      .split(" ")
      .filter((c) => c.startsWith("zoom-"))[0];

    return parseInt(current_zoom.split("-")[1]);
  },
  changeTo(new_zoom_level) {
    if (new_zoom_level < 0) {
      new_zoom_level = 0;
    } else if (new_zoom_level > 7) {
      new_zoom_level = 7;
    }
    let new_zoom = "zoom-" + new_zoom_level;
    let current_zoom = this.currentZoomFromMainElement();
    if (new_zoom != current_zoom) {
      let main = document.getElementById("root_of_all_evil");
      let classes_without_zoom = main.className.replace(/zoom-\d/, "");
      main.className = classes_without_zoom + new_zoom;

      document.cookie = "zoom=" + new_zoom_level + ";SameSite=lax";
    }
  },
  changeBy(offset) {
    let zoom_level = this.currentZoomFromMainElement();
    let new_zoom_level = zoom_level + offset;
    this.changeTo(new_zoom_level);
  },
};

StickySongList = {
  currentWindowDimension() {
    return "{" + window.innerWidth + "x" + window.innerHeight + "}";
  },

  getCurrentScrollPosition() {
    let scrollTop =
      document.documentElement.scrollTop || document.body.scrollTop;
    return this.currentWindowDimension() + "@" + scrollTop;
  },

  storeCurrentScrollPosition() {
    let position = this.getCurrentScrollPosition();
    window.location.hash = position;
  },
  maybeScrollToPositionInLocationHash() {
    const positionFromHash = window.location.hash;
    if (positionFromHash) {
      var [storedDimension, storedPosition] = positionFromHash.split("@");
      if (storedDimension == this.currentWindowDimension()) {
        document.documentElement.scrollTop = document.body.scrollTop =
          parseInt(storedPosition);
      }
    }
  },
  init() {
    document.addEventListener("scrollend", () => {
      this.storeCurrentScrollPosition();
    });

    screen.orientation.addEventListener("change", () => {
      this.storeCurrentScrollPosition();
    });
    this.maybeScrollToPositionInLocationHash();
  },
};

window.addEventListener("load", () => {
  StickySongList.init();
});

Overlay = {
  show(modal_content_id) {
    // hide overlay, hide all possibly open elements in the modal,
    // then show the qr code, then show the overlay
    let overlay = document.getElementById("overlay");
    overlay.dispatchEvent(new Event("hide"));
    for (const content of overlay.getElementsByClassName("modal-content")) {
      if (content.id != modal_content_id) content.classList.add("hidden");
    }
    document.getElementById(modal_content_id).classList.remove("hidden");
    overlay.dispatchEvent(new Event("show"));
  },
  hide() {
    overlay.dispatchEvent(new Event("hide"));
  },
};

Bookmark = {
  isMarked(song_id) {
    let name = "song-" + song_id;
    let cookie_value = Cookies.getValue("bookmarks");
    return cookie_value.split(" ").includes(name);
  },
  toggle(song_id) {
    let name = "song-" + song_id;
    let bookmarked_songs = (Cookies.getValue("bookmarks") || "").split(" ");
    if (bookmarked_songs.includes(name)) {
      bookmarked_songs = bookmarked_songs.filter((e) => e != name);
    } else {
      bookmarked_songs.push(name);
    }
    let new_cookie_value = bookmarked_songs.join(" ");
    Cookies.setValue("bookmarks", new_cookie_value);
  },
};

Toolbar = {
  showQrOverlay() {
    Overlay.show("qr_code");
  },
  hideSearchForm() {
    footer = document.getElementById("footer");
    toggleSearch = document.getElementById("toggle_search");
    searchForm = document.getElementById("search_form");
    searchInput = document.getElementById("search_input");

    toggleSearch.classList.remove("active");
    searchForm.classList.add("hidden");
    footer.classList.remove("show_search_form");
  },
  showSearchForm() {
    footer = document.getElementById("footer");
    toggleSearch = document.getElementById("toggle_search");
    searchForm = document.getElementById("search_form");
    searchInput = document.getElementById("search_input");

    toggleSearch.classList.add("active");
    searchForm.classList.remove("hidden");
    footer.classList.add("show_search_form");
    searchInput.focus();
    searchInput.click();
  },
  toggleSearchForm() {
    searchForm = document.getElementById("search_form");

    if (searchForm.className.split(" ").find((c) => c == "hidden")) {
      this.showSearchForm();
    } else {
      this.hideSearchForm();
    }
  },
  selectCurrentZoomLevel() {
    let zoom_level = "zoom-" + Zoom.currentZoomFromMainElement();
    let choices = document
      .getElementById("zoom_form")
      .getElementsByClassName("zoom-level-choice");
    for (choice of choices) {
      if (choice.className.includes(zoom_level)) {
        choice.classList.add("current-level");
      } else {
        choice.classList.remove("current-level");
      }
    }
  },
  showZoomForm() {
    footer = document.getElementById("footer");
    toggleZoom = document.getElementById("toggle_zoom");
    zoomForm = document.getElementById("zoom_form");

    toggleZoom.classList.add("active");
    zoomForm.classList.remove("hidden");
    this.selectCurrentZoomLevel();
    footer.classList.add("show_zoom_form");
  },
  hideZoomForm() {
    footer = document.getElementById("footer");
    toggleZoom = document.getElementById("toggle_zoom");
    zoomForm = document.getElementById("zoom_form");

    toggleZoom.classList.remove("active");
    zoomForm.classList.add("hidden");
    footer.classList.remove("show_zoom_form");
  },
  toggleZoomForm() {
    zoomForm = document.getElementById("zoom_form");
    if (zoomForm.className.split(" ").find((c) => c == "hidden")) {
      this.showZoomForm();
    } else {
      this.hideZoomForm();
    }
  },

  changeZoomTo(new_level) {
    Zoom.changeTo(new_level);
    this.selectCurrentZoomLevel();
  },

  selectSevenRandomSongs() {
    this.hideSearchForm();
    Overlay.show("feeling_lucky");
    SongList.selectSevenRandomSongs(() => {
      Overlay.hide();
    });
  },
  showSlotMachine() {
    console.log("STUB! Please implement Toolbar.showSlotMachine()");
  },
  toggleArtistList() {
    console.log("STUB! Please implement Toolbar.toggleArtistList()");
  },

  toggleBookmarkSong(song_id) {
    Bookmark.toggle(song_id);
    if (Bookmark.isMarked(song_id)) {
      document.getElementById("bookmark_song").classList.add("active");
    } else {
      document.getElementById("bookmark_song").classList.remove("active");
    }
  },
  toggleShowChords() {
    Chords.toggle();
    if (Chords.showInline()) {
      document.getElementById("toggle_chords").classList.add("active");
    } else {
      document.getElementById("toggle_chords").classList.remove("active");
    }
  },
};
