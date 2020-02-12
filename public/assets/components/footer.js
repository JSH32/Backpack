class Footer extends HTMLElement {
    connectedCallback() {
      this.innerHTML = `
      <div>
          
      <a href="https://github.com/Riku32/Baka.js"><i class='fab fa-github'></i></a>
      &nbsp
      <a href="https://github.com/Riku32/Baka.js/blob/master/README.md"><i class="fas fa-info-circle"></i></a>
      &nbsp
      <a href="/upload"><i class="fas fa-upload"></i></a>
      &nbsp
      <a href="/dash"><i class="fas fa-user-circle"></i></a>

      </div>`;
    }
}
      
customElements.define('footer-area', Footer);