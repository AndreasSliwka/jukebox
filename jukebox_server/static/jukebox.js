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
  Alpine.store("reels", {
    all_tags: [
      "🇬🇧",
      "🇩🇪",
      "🪨",
      "🔨",
      "🛢",
      "🍹",
      "💋",
      "🍦",
      "🎄",
      "👶",
      "60s",
      "70s",
      "80s",
      "90s",
      "00s",
      "10s",
    ],

    first: ["?"],
    second: ["?"],
    third: ["?"],
    randomized_tags() {
      // randomizing the all_tags array every time makes it a bit more random every time
      this.all_tags.sort(() => 0.5 - Math.random());
      return this.all_tags.slice();
    },
    set_up_for_wheelin() {
      this.first = this.first
        .slice(0, 1)
        .concat(this.randomized_tags())
        .concat(this.randomized_tags())
        .slice(0, 30);
      this.second = this.second
        .slice(0, 1)
        .concat(this.randomized_tags())
        .concat(this.randomized_tags())
        .slice(0, 30);
      this.third = this.third
        .slice(0, 1)
        .concat(this.randomized_tags())
        .concat(this.randomized_tags())
        .slice(0, 30);
    },
    copy_last_to_first() {
      this.first[0] = this.first.at(-1);
      this.second[0] = this.second.at(-1);
      this.third[0] = this.third.at(-1);
    },
  });
});

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

Overlay = {
  show(modal_content_id) {
    // hide overlay, hide all possibly open elements in the modal,
    // then show the qr code, then show the overlay
    let overlay = document.getElementById("overlay");
    let modal = document.getElementById("modal");
    overlay.dispatchEvent(new Event("hide"));
    for (const content of overlay.getElementsByClassName("modal-content")) {
      if (content.id != modal_content_id) content.classList.add("hidden");
    }
    let display = document.getElementById(modal_content_id);
    display.classList.remove("hidden");
    if (display.parentElement.id == "modal") {
      modal.classList.remove("hidden");
    } else {
      modal.classList.add("hidden");
    }

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

Slotmachine = {
  show() {
    Overlay.show("slot_machine");
  },
  initialize() {
    Alpine.store("reels").set_up_for_wheelin();
  },
  reroll() {
    let stick = document.getElementById("slotmachine-lever-stick");
    if (stick.classList.contains("working")) return;
    let [reel1, reel2, reel3] = document
      .getElementById("slot_machine")
      .querySelectorAll("ul");
    callback = () => {
      stick.removeEventListener("animationend", callback);
      Alpine.store("reels").copy_last_to_first();
      stick.classList.remove("working");
      reel1.classList.remove("wheelin");
      reel2.classList.remove("wheelin");
      reel3.classList.remove("wheelin");
      Alpine.store("reels").set_up_for_wheelin();
    };
    reel2.addEventListener("animationend", callback);
    stick.classList.add("working");
    reel1.classList.add("wheelin");
    reel2.classList.add("wheelin");
    reel3.classList.add("wheelin");
  },
  selectTag(unicode) {
    if (unicode == "?") return;
    SongList._hide_filtered_out_songs("data-categories", unicode);
    Overlay.hide();
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
    this.hideSearchForm();
    Slotmachine.initialize();
    Slotmachine.show();
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

window.addEventListener("load", () => {
  StickySongList.init();
});
