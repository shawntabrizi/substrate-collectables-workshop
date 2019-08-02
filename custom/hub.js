var substrate_developer_hub_navbar = `
<div class="substrate-dev-hub-nav">
    <a href="https://substrate.dev">
        <img class="logo" 
            src="https://substrate.dev/img/Substrate-logo.svg"
            alt="Substrate Developer Hub">
        <h2 class="headerTitleWithLogo">Substrate Developer Hub</h2>
    </a>
</div>
`;

function addSubstrateDevHubNavBar() {
  var nav_container = document.createElement("div");
  nav_container.innerHTML = substrate_developer_hub_navbar;
  let sidebar = document.getElementsByClassName('sidebar');
  sidebar[0].insertBefore(nav_container, sidebar[0].firstChild);
}

window.onload = function() {
    addSubstrateDevHubNavBar();
};
