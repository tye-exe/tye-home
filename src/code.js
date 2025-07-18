/**
  Used to set a collection of CSS variables globally.
*/
const setCSSVariables = vars => Object.entries(vars).forEach(v => document.querySelector(':root').style.setProperty(v[0], v[1]));
/**
  Collection of light mode CSS settings.
*/
const lightMode = {
  '--global-font-size': '15px',
  '--global-line-height': '1.4em',
  '--global-space': '10px',
  '--font-stack': 'Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, serif',
  '--mono-font-stack': 'Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, serif',
  '--background-color': '#fff',
  '--page-width': '60em',
  '--font-color': '#151515',
  '--invert-font-color': '#fff',
  '--primary-color': '#1a95e0',
  '--secondary-color': '#727578',
  '--error-color': '#d20962',
  '--progress-bar-background': '#727578',
  '--progress-bar-fill': '#151515',
  '--code-bg-color': '#e8eff2',
  '--input-style': 'solid',
  '--display-h1-decoration': 'none',
};
/**
  Collection of dark mode CSS settings.
*/
const darkMode = {
  '--global-font-size': '15px',
  '--global-line-height': '1.4em',
  '--global-space': '10px',
  '--font-stack': 'Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace, serif',
  '--mono-font-stack': 'Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace, serif',
  '--background-color': '#222225',
  '--page-width': '60em',
  '--font-color': '#e8e9ed',
  '--invert-font-color': '#222225',
  '--secondary-color': '#a3abba',
  '--tertiary-color': '#a3abba',
  '--primary-color': '#62c4ff',
  '--error-color': '#ff3c74',
  '--progress-bar-background': '#3f3f44',
  '--progress-bar-fill': '#62c4ff',
  '--code-bg-color': '#3f3f44',
  '--input-style': 'solid',
  '--display-h1-decoration': 'none',
};

function set_light() {
  setCSSVariables(lightMode);
}

function set_dark() {
  setCSSVariables(darkMode);
}

// Toggle sidebar between hidden and shown on mobile.
function show_mobile_menu() {
  let sidebar = document.getElementById("sidebar");

  let to_add = "sidebar";
  let to_remove = "sidebar-mobile";

  if (sidebar.classList.contains("sidebar")) {
    to_add = "sidebar-mobile";
    to_remove = "sidebar"
  }

  sidebar.classList.add(to_add);
  sidebar.classList.remove(to_remove);
}
