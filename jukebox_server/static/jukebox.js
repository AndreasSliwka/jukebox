class AtticSongList {
  // companion object to <div id='songlist'>, which contains
  filterByName(raw_term) {
    let term = raw_term.toLowerCase();

    document.cookie = "search=" + term.replaceAll("'", "\\'") + ";SameSite=lax";
    document.cookie = "category=;SameSite=lax";
    // deselect_category_wof_entries();
    this._hide_filtered_out_songs("data-name", raw_term.toLowerCase());
    if (raw_term == "") {
      Toolbar.hideSearchForm();
    }
  }
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
  }
}
AtticSongList = new AtticSongList();

class Song {
  last_clicked_id = null;
  last_click_when = 0;

  part_clicked(event) {
    let element = event.target.closest(".chordpro");
    console.log(event.target);
    let clicked_when = Date.now();
    if (
      element &&
      element.id == this.last_clicked_id &&
      this.last_click_when + 500 > clicked_when
    ) {
      let highlighted = document.getElementsByClassName("scrollHighlight");
      for (const part of highlighted) {
        song.classList.remove("scrollHighlight");
      }
      target_y = element.getBoundingClientRect().y;
      target_y -= document.querySelector("header").clientHeight;
      target_y -= 30;
      window.scrollTo(0, target_y);
      element.classList.add("scrollHighlight");

      this.last_clicked_id = null;
    } else {
      this.last_clicked_id = element.id;
      this.last_click_when = clicked_when;
    }
  }
}
Song = new Song();

class AllSongs {
  all_songs = [];
  all_artists = [];

  // data stuff
  initialize(songs) {
    this.all_songs = songs.map((song) => {
      song.id = "song-" + song.id;
      return song;
    });
    let intermediate = songs.reduce(function (acc, song) {
      acc[song.artist] = 1;
      return acc;
    }, {});

    this.all_artists = Object.keys(intermediate).sort();
  }
  songsByIds(songs_ids) {
    return this.all_songs.filter((song) => songs_ids.includes(song.id));
  }
  songById(song_id) {
    return this.songsByIds([song_id])[0];
  }
}
AllSongs = new AllSongs();

class Cookies {
  getValue(cookieName) {
    let match = document.cookie.match(new RegExp(cookieName + "=([^;]*)(;|$)"));
    let value;
    if (match) {
      value = match[1];
    } else {
      value = "";
    }
    return value;
  }
  setValue(cookieName, value) {
    document.cookie = cookieName + "=" + value + ";SameSite=lax";
  }
  storedShowChords() {
    return this.getValue("showChords") == "true";
  }
}
Cookies = new Cookies();

class Chords {
  showInline() {
    return Cookies.getValue("showChords") == "true";
  }
  toggle() {
    if (this.showInline()) {
      Cookies.setValue("showChords", "false");
    } else {
      Cookies.setValue("showChords", "true");
    }
    this.maybeShow();
  }
  maybeShow() {
    let song = window.document.getElementById("song");
    if (song) {
      if (this.showInline()) {
        song.classList.add("showChords");
      } else {
        song.classList.remove("showChords");
      }
    }
  }
}
Chords = new Chords();

function deselect_category_wof_entries() {
  Array.from(
    document
      .getElementById("wheel_of_fortune")
      .getElementsByClassName("category selected"),
  ).forEach((category) => {
    category.classList.remove("selected");
  });
}

class Zoom {
  currentZoomFromMainElement() {
    let main = document.getElementById("root_of_all_evil");

    let current_zoom = main.className
      .split(" ")
      .filter((c) => c.startsWith("zoom-"))[0];

    return parseInt(current_zoom.split("-")[1]);
  }
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
  }
  changeBy(offset) {
    let zoom_level = this.currentZoomFromMainElement();
    let new_zoom_level = zoom_level + offset;
    this.changeTo(new_zoom_level);
  }
}
Zoom = new Zoom();

class StickySongList {
  currentWindowDimension() {
    return "{" + window.innerWidth + "x" + window.innerHeight + "}";
  }
  getCurrentScrollPosition() {
    let scrollTop =
      document.documentElement.scrollTop || document.body.scrollTop;
    return this.currentWindowDimension() + "@" + scrollTop;
  }
  storeCurrentScrollPosition() {
    let position = this.getCurrentScrollPosition();
    window.location.hash = position;
  }
  maybeScrollToPositionInLocationHash() {
    const positionFromHash = window.location.hash;
    if (positionFromHash) {
      var [storedDimension, storedPosition] = positionFromHash.split("@");
      if (storedDimension == this.currentWindowDimension()) {
        document.documentElement.scrollTop = document.body.scrollTop =
          parseInt(storedPosition);
      }
    }
  }
  init() {
    document.addEventListener("scrollend", () => {
      this.storeCurrentScrollPosition();
    });

    screen.orientation.addEventListener("change", () => {
      this.storeCurrentScrollPosition();
    });
    this.maybeScrollToPositionInLocationHash();
  }
}
StickySongList = new StickySongList();

class Overlay {
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
    overlay.classList.remove("hidden");
  }
  hide() {
    Toolbar.onlyActivateToolButton("no such button");
    overlay.classList.add("hidden");
  }
}
Overlay = new Overlay();

class Bookmark {
  isMarked(song_id) {
    let handle = "song-" + song_id;
    return this.bookmarkedSongIds().includes(handle);
  }
  toggle(song_id) {
    let name = "song-" + song_id;
    let bookmarked_song_ids = this.bookmarkedSongIds();
    if (bookmarked_song_ids.includes(name)) {
      bookmarked_song_ids = bookmarked_song_ids.filter((e) => e != name);
    } else {
      bookmarked_song_ids.push(name);
    }
    let new_cookie_value = bookmarked_song_ids.join(",");
    Cookies.setValue("bookmarks", new_cookie_value);
  }
  bookmarkedSongIds() {
    return (Cookies.getValue("bookmarks") || "").split(",");
  }
  bookmarkedSongs() {
    let song_ids = this.bookmarkedSongIds();
    let all_songs = AllSongs.songsByIds(song_ids);
    return all_songs;
  }
}
Bookmark = new Bookmark();
class ReelsStore {
  // TODO: refactor to make this an initialisation paramter.
  // TODO: The initialisation can than be called from rust produced code,
  // TODO: thus being always up-to-date.
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
    "🍽",
    "🐞",
    "🚑",
    "🎨",
    "🔥",
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
}


class SongListStore {
  visible: [],
  update(newCurrentSongs) {
    this.visible = newCurrentSongs.slice();
  },
  allSongs() {
    console.log("$store.songlist.allSongs()");
    this.update(AllSongs.all_songs);
  },
  setTextFilter(original_term) {
    let term = original_term.toLowerCase();
    let filtered_songs = AllSongs.all_songs.filter(
      (song) =>
        song.title.toLowerCase().includes(term) ||
        song.artist.toLowerCase().includes(term),
    );
    this.update(filtered_songs);
  },
  setCategoryFilter(tag) {
    let filtered_songs = AllSongs.all_songs.filter((song) =>
      song.tag_signs.includes(tag),
    );
    this.update(filtered_songs);
  },
  pushSong(song) {
    this.visible.push(song);
  },
  filterByArtist(artist) {
    let filtered_songs = AllSongs.all_songs.filter(
      (song) => song.artist == artist,
    );
    this.update(filtered_songs);
  },

  selectSevenRandomSongs(andThen = () => {}) {
    this.update([]);
    const store = this;
    randomized_songs = AllSongs.all_songs
      .toSorted(() => 0.5 - Math.random())
      .sort(() => 0.5 - Math.random());
    selected_ids = randomized_songs
      .map((song) => song.id)
      .toSorted(() => 0.5 - Math.random())
      .slice(0, 7);
    function maybe_show_select_song() {
      if (randomized_songs.length > 0) {
        let song = randomized_songs.pop();
        if (selected_ids.includes(song.id)) {
          store.pushSong(song);
        }
        setTimeout(maybe_show_select_song, 6);
      } else {
        andThen();
      }
    }

    maybe_show_select_song();
  },
}
document.addEventListener("alpine:init", () => {
  Alpine.store("reels", new ReelsStore() );
  Alpine.store("songlist", new SongListStore() );
});

class ArtistList {
  filterByArtist(li_element) {
    let previous_selected_element = document.querySelector(
      "#songlist_container #artist_list li.selected",
    );
    if (previous_selected_element) {
      previous_selected_element.classList.remove("selected");
    }
    li_element.classList?.add("selected");
    Alpine.store("songlist").filterByArtist(li_element.textContent);
  }
}
ArtistList = new ArtistList();

class Slotmachine {
  show() {
    Overlay.show("slot_machine");
  }
  initialize() {
    Alpine.store("reels").set_up_for_wheelin();
  }
  reroll() {
    let stick = document.getElementById("slotmachine-lever-stick");
    if (stick.classList.contains("working")) return;
    let [reel1, reel2, reel3] = document
      .getElementById("slot_machine")
      .querySelectorAll("ul");
    // we use reel 2 for animation-end, as its the
    // last animation to be running.
    let callback = () => {
      reel2.removeEventListener("animationend", callback);
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
  }
  selectTag(unicode) {
    if (unicode == "?") return;
    Overlay.hide();
    Header.showWhichCategory(unicode);
    Toolbar.onlyActivateToolButton("show_slot_machine");
    Alpine.store("songlist").setCategoryFilter(unicode);
  }
}
Slotmachine = new Slotmachine();

class Header {
  showWhichCategory(unicode) {
    let category_info = document.getElementById("which_category");

    category_info.classList.remove("hidden");
    category_info.getElementsByClassName("category")[0].textContent = unicode;
  }
  showTitle(text) {
    document.getElementById("which_category").classList.add("hidden");
    document.getElementById("title").textContent = text;
  }
}
Header = new Header();

class Toolbar {
  onlyActivateToolButton(button_id) {
    let buttons = document.querySelectorAll("#footer a.toolbar-button");

    for (let tool_button of buttons) {
      if (tool_button.id == button_id) {
        tool_button.classList.add("active");
      } else {
        tool_button.classList.remove("active");
      }
    }
  }
  showAllSongs() {
    this.onlyActivateToolButton("all_songs");
    Alpine.store("songlist").allSongs();
    Header.showTitle("Alle Songs");
  }
  showQrOverlay() {
    Overlay.show("qr_code");
  }
  hideCategoryFilter() {
    document.getElementById("which_category").classList.add("hidden");
  }
  hideSearchForm() {
    let footer = document.getElementById("footer");
    let toggleSearch = document.getElementById("toggle_search");
    let searchForm = document.getElementById("search_form");
    let searchInput = document.getElementById("search_input");

    toggleSearch.classList.remove("active");
    searchForm.classList.add("hidden");
    footer.classList.remove("show_search_form");
  }
  showSearchForm() {
    let footer = document.getElementById("footer");
    let toggleSearch = document.getElementById("toggle_search");
    let searchForm = document.getElementById("search_form");
    let searchInput = document.getElementById("search_input");

    this.onlyActivateToolButton("toggle_search");

    searchForm.classList.remove("hidden");
    footer.classList.add("show_search_form");
    searchInput.focus();
    searchInput.click();
  }
  toggleSearchForm() {
    this.hideArtistList();
    let searchForm = document.getElementById("search_form");

    if (searchForm.className.split(" ").find((c) => c == "hidden")) {
      this.showSearchForm();
    } else {
      this.hideSearchForm();
      Alpine.store("songlist").allSongs();
    }
  }
  setSearchFilter(target) {
    this.hideCategoryFilter();
    Alpine.store("songlist").setTextFilter(target);
  }
  selectCurrentZoomLevel() {
    let zoom_level = "zoom-" + Zoom.currentZoomFromMainElement();
    let choices = document
      .getElementById("zoom_form")
      .getElementsByClassName("zoom-level-choice");
    for (let choice of choices) {
      if (choice.className.includes(zoom_level)) {
        choice.classList.add("current-level");
      } else {
        choice.classList.remove("current-level");
      }
    }
  }
  showZoomForm() {
    let footer = document.getElementById("footer");
    let toggleZoom = document.getElementById("toggle_zoom");
    let zoomForm = document.getElementById("zoom_form");

    toggleZoom.classList.add("active");
    zoomForm.classList.remove("hidden");
    this.selectCurrentZoomLevel();
    footer.classList.add("show_zoom_form");
  }
  hideZoomForm() {
    let footer = document.getElementById("footer");
    let toggleZoom = document.getElementById("toggle_zoom");
    let zoomForm = document.getElementById("zoom_form");

    toggleZoom.classList.remove("active");
    zoomForm.classList.add("hidden");
    footer.classList.remove("show_zoom_form");
  }
  toggleZoomForm() {
    let zoomForm = document.getElementById("zoom_form");
    if (zoomForm.className.split(" ").find((c) => c == "hidden")) {
      this.showZoomForm();
    } else {
      this.hideZoomForm();
    }
  }
  changeZoomTo(new_level) {
    Zoom.changeTo(new_level);
    this.selectCurrentZoomLevel();
  }
  selectSevenRandomSongs() {
    Overlay.show("feeling_lucky");
    Header.showTitle("Feeling Lucky?");
    this.hideSearchForm();
    this.hideArtistList();
    this.hideCategoryFilter();
    this.onlyActivateToolButton("select_four_random_songs");
    Alpine.store("songlist").selectSevenRandomSongs(() => {
      Overlay.hide();
      this.onlyActivateToolButton("select_four_random_songs");
    });
  }
  showSlotMachine() {
    this.hideSearchForm();
    this.hideArtistList();
    this.onlyActivateToolButton("show_slot_machine");
    Slotmachine.initialize();
    Slotmachine.show();
  }
  hideArtistList() {
    document
      .getElementById("root_of_all_evil")
      .classList.remove("show_artists");
    // Alpine.store("songlist").allSongs();
  }
  toggleArtistList() {
    this.hideSearchForm();
    this.onlyActivateToolButton("toggle_show_artists");
    document.getElementById("root_of_all_evil").classList.add("show_artists");
    ArtistList.filterByArtist("");
  }
  toggleBookmarkSong(song_id) {
    Bookmark.toggle(song_id);
    if (Bookmark.isMarked(song_id)) {
      document.getElementById("bookmark_song").classList.add("active");
    } else {
      document.getElementById("bookmark_song").classList.remove("active");
    }
  }
  toggleShowChords() {
    Chords.toggle();
    if (Chords.showInline()) {
      document.getElementById("toggle_chords").classList.add("active");
    } else {
      document.getElementById("toggle_chords").classList.remove("active");
    }
  }
}
Toolbar = new Toolbar();

window.addEventListener("load", () => {
  StickySongList.init();
});
