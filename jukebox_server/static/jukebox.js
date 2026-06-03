class Song {
  last_clicked_id = null;
  last_click_when = 0;

  part_clicked(event) {
    let element = event.target.closest(".chordpro");
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
  async markAsPlayed(button, value) {
    button.disabled = true;
    let url = document.location.href + "?add_to_setlist=" + value;
    await fetch(url);
    setTimeout(() => {
      if (value == 1) {
        button.parentElement.classList.add("played");
      } else {
        button.parentElement.classList.remove("played");
      }
    }, 1000);
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
  show(modal_content_id, hide_others = true) {
    // hide overlay, hide all possibly open elements in the modal,
    // then show the qr code, then show the overlay
    let overlay = document.getElementById("overlay");
    let modal = document.getElementById("modal");
    overlay.dispatchEvent(new Event("hide"));
    if (hide_others) {
      for (const content of overlay.getElementsByClassName("modal-content")) {
        if (content.id != modal_content_id) content.classList.add("hidden");
      }
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
    let overlay = document.getElementById("overlay");

    Toolbar.onlyActivateToolButton("no such button");
    overlay.classList.add("hidden");
  }
  async showAdminQR(gig_id) {
    const response = await fetch(`/gigs/${gig_id}/admin_qr`);
    const qr_code = await response.json();
    Alpine.store("gigs").qr_code = qr_code;
    const modal = document.getElementById("admin_qr_code");
    modal.classList.remove("hidden");
  }
  hideAdminQR() {
    const modal = document.getElementById("admin_qr_code");
    modal.classList.add("hidden");
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
  all_tags = [];

  first = ["?"];
  second = ["?"];
  third = ["?"];
  setAllTags(allTags) {
    this.all_tags = allTags;
  }
  randomized_tags() {
    // randomizing the all_tags array every time makes it a bit more random every time
    this.all_tags.sort(() => 0.5 - Math.random());
    return this.all_tags.slice();
  }
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
  }
  copy_last_to_first() {
    this.first[0] = this.first.at(-1);
    this.second[0] = this.second.at(-1);
    this.third[0] = this.third.at(-1);
  }
}

class SongListStore {
  visible = [];
  update(newCurrentSongs) {
    this.visible = newCurrentSongs.slice();
  }
  allSongs() {
    this.update(AllSongs.all_songs);
  }
  setTextFilter(original_term) {
    let term = original_term.toLowerCase();
    let filtered_songs = AllSongs.all_songs.filter(
      (song) =>
        song.title.toLowerCase().includes(term) ||
        song.artist.toLowerCase().includes(term),
    );
    this.update(filtered_songs);
  }
  setCategoryFilter(tag) {
    let filtered_songs = AllSongs.all_songs.filter((song) =>
      song.tag_signs.includes(tag),
    );
    this.update(filtered_songs);
  }
  pushSong(song) {
    this.visible.push(song);
  }
  visibleSongIds() {
    return this.visible.map((song) => song.id);
  }
  filterByArtist(artist) {
    let filtered_songs = AllSongs.all_songs.filter(
      (song) => song.artist == artist,
    );
    this.update(filtered_songs);
  }
  filterPlayedSongs(played) {
    let filtered_songs = AllSongs.all_songs
      .filter((song) => song.played_at != "")
      .toSorted((a, b) => a.played_at.localeCompare(b.played_at));
    console.this.update(filtered_songs);
  }
  selectSevenRandomSongs(andThen = () => {}, prepicked_songs = "") {
    this.update([]);
    const store = this;
    var selected_ids;
    const randomized_songs = AllSongs.all_songs
      .toSorted(() => 0.5 - Math.random())
      .sort(() => 0.5 - Math.random());

    if (prepicked_songs) {
      selected_ids = prepicked_songs.split(",");
    } else {
      selected_ids = randomized_songs
        .map((song) => song.id)
        .toSorted(() => 0.5 - Math.random())
        .slice(0, 7);
    }

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
  }
}
class Secret {
  static zahlen = [
    "zwei",
    "drei",
    "vier",
    "fuenf",
    "sechs",
    "sieben",
    "acht",
    "neun",
    "zehn",
    "elf",
  ];
  static farben = [
    "rote",
    "gruene",
    "blaue",
    "gelbe",
    "violette",
    "braune",
    "schwarze",
    "weisse",
    "graue",
    "orange",
  ];
  static tiere = [
    "enten",
    "schweine",
    "hunde",
    "katzen",
    "maeuse",
    "fische",
    "alpacas",
  ];
  static random_secret() {
    return (
      Secret.zahlen[Math.floor(Math.random() * Secret.zahlen.length)] +
      "-" +
      Secret.farben[Math.floor(Math.random() * Secret.farben.length)] +
      "-" +
      Secret.tiere[Math.floor(Math.random() * Secret.tiere.length)]
    );
  }
}
class GigsStore {
  selected_gig = null;
  all_gigs = [];
  last_click_when = false;
  qr_code = {};
  can_edit_gig = false;
  selectGig(gig_id) {
    let original = this.all_gigs.find((gig) => gig.id === gig_id);
    let gig = JSON.parse(JSON.stringify(original));
    gig.errors = {};
    gig.show_private = gig.show_private == 1;
    gig.default_gig = gig.default_gig == 1;

    this.selected_gig = gig;
    this.can_edit_gig = gig.id === this.all_gigs[0].id && !gig.default_gig;
  }
  unselectGig() {
    this.selected_gig = null;
  }
  createNewGig() {
    this.selected_gig = {
      id: "none",
      name: "",
      location: "",
      date_start: "",
      date_end: "",
      admin_secret: Secret.random_secret(),
      notes: "",
      show_private: false,
      default_gig: false,
      errors: {},
    };
    this.can_edit_gig = true;
  }
  async parseResponse(response) {
    const json = await response.json();

    this.all_gigs = json.all_gigs;
    if (json.updated_gig) {
      this.selected_gig = json.updated_gig;
      this.selected_gig.errors = {};
    } else if (json.current_gig) {
      this.selected_gig = json.current_gig;
      this.selected_gig.errors = {};
    }

    this.last_gig_not_finished = this.all_gigs[0].date_end === "";
    this.can_edit_gig =
      this.selected_gig &&
      this.selected_gig.id === this.all_gigs[0].id &&
      !this.selected_gig.default_gig;
  }
  async loadGigs() {
    const response = await fetch("/gigs");

    await this.parseResponse(response);
  }
  gig_has_errors() {
    this.selected_gig.name = document.getElementById("gig-name").value;
    this.selected_gig.location = document.getElementById("gig-location").value;
    this.selected_gig.notes = document.getElementById("gig-notes").value;

    const editable_secret = document.getElementById("gig-secret");
    if (editable_secret) {
      this.selected_gig.admin_secret = editable_secret.value;
    }

    this.selected_gig.errors = {};
    if (this.selected_gig.name == "") {
      this.selected_gig.errors.name = "Name darf nicht leer sein";
    }
    if (this.selected_gig.location == "") {
      this.selected_gig.errors.location = "Ort darf nicht leer sein";
    }
    if (this.selected_gig.notes == "") {
      this.selected_gig.errors.notes = "Notizen dürfen nicht leer sein";
    }
    if (this.selected_gig.admin_secret == "") {
      this.selected_gig.errors.admin_secret = "Passwort darf nicht leer sein";
    }

    if (Object.keys(this.selected_gig.errors).length) {
      return true;
    }
    return false;
  }
  async transmitGig(method = "POST") {
    const gig = JSON.parse(JSON.stringify(this.selected_gig));
    gig.show_private = gig.show_private ? 1 : 0;
    gig.default_gig = gig.default_gig ? 1 : 0;
    const response = await fetch("/gigs", {
      method: method,
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(gig),
    });
    await this.parseResponse(response);
  }
  async startSelectedGig() {
    if (this.gig_has_errors()) {
      return;
    }

    this.selected_gig.date_start = "now";
    let method = "POST";
    if (this.selected_gig.id === "none") {
      method = "PUT";
    }
    await this.transmitGig(method);
  }
  async stopSelectedGig() {
    if (this.gig_has_errors()) {
      return;
    }

    this.selected_gig.date_end = "now";
    await this.transmitGig();
  }
  async unstopSelectedGig() {
    if (this.gig_has_errors()) {
      return;
    }

    this.selected_gig.date_end = "";
    await this.transmitGig();
  }
  async songsPlayedInSelectedGig() {
    if (this.selected_gig.id === "none") {
      return [];
    }
    const response = await fetch(`/gig/${this.selected_gig.id}/songs`);
    const json = await response.json();
    this.selected_gig.songs = json;
    return json;
  }
}

document.addEventListener("alpine:init", () => {
  Alpine.store("reels", new ReelsStore());
  Alpine.store("songlist", new SongListStore());
  Alpine.store("gigs", new GigsStore());
});

class ArtistList {
  filterByArtist(artist_name) {
    let previous_selected_element = document.querySelector(
      "#songlist_container #artist_list li.selected",
    );
    if (previous_selected_element) {
      previous_selected_element.classList.remove("selected");
    }
    const li_element = [].slice
      .call(document.querySelectorAll("#songlist_container #artist_list li"))
      .filter(function (el) {
        return el.textContent === artist_name;
      })[0];
    li_element?.classList.add("selected");
    li_element?.scrollIntoView({ behavior: "instant" });
    Alpine.store("songlist").filterByArtist(artist_name);
    Header.showWhichCategory(artist_name);
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
    SongListToolbar.onlyActivateToolButton("show_slot_machine");
    Alpine.store("songlist").setCategoryFilter(unicode);
  }
}
Slotmachine = new Slotmachine();

class Header {
  showWhichCategory(unicode) {
    if (!unicode) return;
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
  showQrOverlay() {
    Overlay.show("qr_code");
  }
  showAdminPanel() {
    Alpine.store("gigs").loadGigs();
    Overlay.show("admin_panel");
  }
}

class SongListToolbar extends Toolbar {
  states = {
    AllSongs: "all",
    Search: "search",
    ArtistList: "artistList",
    PlayedSongs: "playedSongs",
    SlotMachine: "slotMachine",
    SevenRandomSongs: "randomSongs",
  };
  current_state = this.states.AllSongs;
  hideCategoryFilter() {
    document.getElementById("which_category").classList.add("hidden");
  }
  parseStateFromUrlOrCookie() {
    const url = new URL(window.location.href);
    if (url.searchParams.size != 0) {
      let state = url.searchParams.keys().next().value;
      let specifics = url.searchParams.get(state);

      return this.switchToState(state, specifics);
    }
    this.restoreFromCookie();
  }
  switchToState(state, specifics = "") {
    switch (state) {
      case this.states.AllSongs:
        this.showAllSongs();
        break;
      case this.states.Search:
        if (specifics.startsWith("admin:")) {
          this.showSearchForm("");
        } else {
          this.showSearchForm(specifics);
        }
        break;
      case this.states.ArtistList:
        this.showArtistList(specifics);
        break;
      case this.states.PlayedSongs:
        this.showPlayedSongs(specifics);
        break;
      case this.states.SlotMachine:
        this.showSlotMachine(specifics);
        break;
      case this.states.SevenRandomSongs:
        this.selectSevenRandomSongs(specifics);
        break;
      default:
        this.showAllSongs();
        return;
    }
  }
  saveState(state, info = 1) {
    Cookies.setValue("state", JSON.stringify({ [state]: info }));
  }
  restoreFromCookie() {
    let state = Cookies.getValue("state");
    if (!state) return;

    state = JSON.parse(state);
    let key = Object.keys(state)[0];
    this.switchToState(key, state[key]);
  }
  showAllSongs() {
    this.current_state = this.states.AllSongs;
    this.hideSearchForm();
    this.hideArtistList();
    this.hideCategoryFilter();
    this.onlyActivateToolButton("all_songs");
    Alpine.store("songlist").allSongs();
    Header.showTitle("Alle Songs");
    this.saveState(this.states.AllSongs);
  }
  hideSearchForm() {
    let footer = document.getElementById("footer");
    let toggleSearch = document.getElementById("toggle_search");
    let searchForm = document.getElementById("search_form");

    toggleSearch.classList.remove("active");
    searchForm.classList.add("hidden");
    footer.classList.remove("show_search_form");
  }
  showSearchForm(preloaded_search_term = "") {
    let footer = document.getElementById("footer");
    let searchForm = document.getElementById("search_form");
    let searchInput = document.getElementById("search_input");
    this.hideArtistList();
    this.hideCategoryFilter();
    this.onlyActivateToolButton("toggle_search");

    searchForm.classList.remove("hidden");
    footer.classList.add("show_search_form");
    searchInput.focus();
    searchInput.click();
    searchInput.value = preloaded_search_term;
    this.setSearchFilter(preloaded_search_term);
  }
  setSearchFilter(original_term) {
    // called by changes from the search input
    if (original_term.startsWith("admin:") && original_term.endsWith("!")) {
      let passkey = original_term.replace(/^admin:/, "").replace(/!$/, "");
      window.location.href = "/admin?passkey=" + passkey;
    }
    Alpine.store("songlist").setTextFilter(original_term);
    this.saveState(this.states.Search, original_term);
  }

  selectSevenRandomSongs(prepicked_songs) {
    Overlay.show("feeling_lucky");
    Header.showTitle("Feeling Lucky?");
    this.hideSearchForm();
    this.hideArtistList();
    this.hideCategoryFilter();
    this.onlyActivateToolButton("select_seven_random_songs");
    Alpine.store("songlist").selectSevenRandomSongs(() => {
      Overlay.hide();
      this.onlyActivateToolButton("select_seven_random_songs");
      this.saveState(
        this.states.randomSongs,
        Alpine.store("songlist").visibleSongIds(),
      );
    }, prepicked_songs);
  }
  showSlotMachine() {
    this.hideSearchForm();
    this.hideArtistList();
    this.hideCategoryFilter();
    // this.onlyActivateToolButton("show_slot_machine");
    Slotmachine.initialize();
    Slotmachine.show();
  }
  hideArtistList() {
    document
      .getElementById("root_of_all_evil")
      .classList.remove("show_artists");
    // Alpine.store("songlist").allSongs();
  }
  showArtistList(preloaded_artist = "") {
    this.hideSearchForm();
    this.hideCategoryFilter();
    this.onlyActivateToolButton("toggle_show_artists");
    document.getElementById("root_of_all_evil").classList.add("show_artists");
    ArtistList.filterByArtist(preloaded_artist);
  }
  showPlayedSongs(preloaded_played = "") {
    this.hideSearchForm();
    this.hideCategoryFilter();
    this.onlyActivateToolButton("filter_played_songs");
    Alpine.store("songlist").filterPlayedSongs(preloaded_played);
    this.saveState(this.states.PlayedSongs);
    Header.showWhichCategory("gespielten");
  }
}

class SingleSongToolbar extends Toolbar {
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
  backToSongList() {
    let song_list_url = new URL("/songs", window.location.href);
    for (const key in history.state) {
      song_list_url.searchParams.set(key, history.state[key]);
    }
    window.location.href = song_list_url.toString();
  }
}

Toolbar = new Toolbar();
SongListToolbar = new SongListToolbar();
SingleSongToolbar = new SingleSongToolbar();

function only_hours_and_minutes(played_at) {
  return played_at.replace(/.*T(..:..).*/, "$1");
}
