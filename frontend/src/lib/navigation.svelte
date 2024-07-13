<script>
  let visible = false;

  function toggleVisible() {
    visible = !visible;
  }
</script>

<div id="top-bar">
  <img src="deer-silhouette.png"/>
  <p>Edwin Wallin</p>
  <button on:click={toggleVisible}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--!Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M0 96C0 78.3 14.3 64 32 64H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z"/></svg>    
  </button>
</div>

{#if visible}
  <nav>
    <ul>
      <li><a href="/">Home</a></li>
      <li><a href="/about/">About</a></li>
      <li><a href="/store/1">Store</a></li>
    </ul>
  </nav>
{/if}


<style>
  #top-bar {
    color: #FFF;

    width: 100%;
    height: 5%;
    background-color: #333;
    padding: 16px;

    display: flex;
    align-items: center;
    justify-content: space-between;

  }
  
  #top-bar img {
    filter: invert(1);
    margin-right: 16px;
  }

  #top-bar p {
    font-size: 1.2em;
  }

  #top-bar button {
    background-color: #333;
    border: none;
  }

  #top-bar button svg {
    filter: invert(1);
  }

  #top-bar button, #top-bar img {
    width: 4.2vh;
    height: 4.2vh;
  }


  #top-bar-left {
    width: 90%;
  }

  nav {
    position: absolute;
    width: 100%;
    height: 95%;

    background-color: #333;
  }

  ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
    overflow: hidden;
  }
  
  li a {
    display: block;
    color: white;
    text-align: center;
    padding: 14px 16px;
    text-decoration: none;
  }
  
  li a:hover {
    background-color: #111;
  }
</style>
