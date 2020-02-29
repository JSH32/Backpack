class Footer extends HTMLElement {
    connectedCallback() {
      this.innerHTML = `
      <div>
          
      <a href="/"><i class="fas fa-home"></i></a>
      &nbsp
      <a href="/about"><i class="fas fa-info-circle"></i></a>
      &nbsp
      <a href="/upload"><i class="fas fa-upload"></i></a>
      &nbsp
      <a href="/dash"><i class="fas fa-user-circle"></i></a>

      </div>`;
    }
}
      
customElements.define('footer-area', Footer);