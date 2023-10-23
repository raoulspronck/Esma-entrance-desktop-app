<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { exit } from "@tauri-apps/api/process";

  let time = new Date().getTime();
  let intervalTime = 0;

  // checks connection
  onMount(() => {
    listen("ping", (event) => {
      time = new Date().getTime();
      intervalTime = 0;
    });
  });

  setInterval(() => {
    intervalTime = Math.round((new Date().getTime() - time) / 1000);
  }, 1000);
</script>

<main class="main" style="position: relative;">
  <div style="position: absolute; top: 0px; right: 0px; ">
    <button
      style="width: 50px; height: 50px; background-color: transparent; border: none;"
      on:click={async () => await exit(1)}
    >
      -{intervalTime}
    </button>
  </div>

  {#if intervalTime < 10}
    <div class="aanmeld-row">
      <div class="language-row">
        <img src="/icons8-netherlands-96.png" alt="Nederland" />
        <div class="language-text">
          <h2>Welkom bij Esma!</h2>
          <a href="/nl">
            <button class="aanmeld-button"
              >Klik hier alstublieft om u aan te melden.</button
            >
          </a>
        </div>
      </div>

      <div class="language-row">
        <img src="/icons8-great-britain-96.png" alt="UK" />
        <div class="language-text">
          <h2>Welcome at Esma!</h2>
          <a href="/en">
            <button class="aanmeld-button"
              >Please, click here to register.</button
            >
          </a>
        </div>
      </div>

      <div class="language-row">
        <img src="/icons8-germany-96.png" alt="UK" />
        <div class="language-text">
          <h2>Willkommen bei Esma!</h2>
          <a href="/ge">
            <button class="aanmeld-button"
              >Bitte klicken Sie hier, um sich zu registrieren.</button
            >
          </a>
        </div>
      </div>

      <div class="language-row">
        <img src="/icons8-france-96.png" alt="France" />
        <div class="language-text">
          <h2>Bonjour chez Esma!</h2>
          <a href="/fr">
            <button class="aanmeld-button"
              >Cliquez ici pour vous inscrire, merci.</button
            >
          </a>
        </div>
      </div>

      <div class="language-row">
        <img src="/icons8-poland-96.png" alt="Poland" />
        <div class="language-text">
          <h2>Witamy w Esma!</h2>
          <a href="/po">
            <button class="aanmeld-button"
              >Kliknij tutaj, aby się zarejestrować.</button
            >
          </a>
        </div>
      </div>

      <div class="language-row">
        <img src="/icons8-romania-96.png" alt="Romania" />
        <div class="language-text">
          <h2>Bun venit la Esma!</h2>
          <a href="/ro">
            <button class="aanmeld-button"
              >Vă rugăm să faceți clic aici pentru a vă înregistra.</button
            >
          </a>
        </div>
      </div>

      <div class="language-row">
        <img src="/icons8-bulgaria-96.png" alt="Bulgaria" />
        <div class="language-text">
          <h2>Добре дошла при Есма!</h2>
          <a href="/bu">
            <button class="aanmeld-button"
              >Моля, щракнете тук, за да се регистрирате.</button
            >
          </a>
        </div>
      </div>
    </div>
  {/if}
</main>

<style>
  .main {
    margin: 0;
    padding: 0;

    background-image: url("/Welcome-screen-esma.png");

    /* Full height */
    height: 100vh;
    width: 100vw;

    /* Center and scale the image nicely */
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
  }

  .aanmeld-row {
    padding-top: 10px;
    padding-left: 30px;
  }

  .language-row {
    display: flex;
    align-items: center;
    margin-bottom: 10px;
  }

  .language-row img {
    height: 96px;
    width: 96px;
    margin-right: 20px;
  }

  .language-text {
    text-align: left;
    line-height: 0px;
  }

  .language-text h2 {
    font-size: 35px;
  }

  .aanmeld-button {
    font-size: 20px;
    font-weight: 400;
    background-color: initial;
    background-image: linear-gradient(-180deg, #c82c3e, #a02c3f);
    border-radius: 6px;
    box-shadow: rgba(0, 0, 0, 0.1) 0 2px 4px;
    color: #ffffff;
    cursor: pointer;
    display: inline-block;
    font-family: Inter, -apple-system, system-ui, Roboto, "Helvetica Neue",
      Arial, sans-serif;
    height: 50px;
    line-height: 50px;
    outline: 0;
    overflow: hidden;
    padding: 0 20px;
    pointer-events: auto;
    position: relative;
    text-align: center;
    touch-action: manipulation;
    user-select: none;
    -webkit-user-select: none;
    vertical-align: top;
    white-space: nowrap;
    width: fit-content;
    z-index: 9;
    border: 0;
    transition: box-shadow 0.2s;
  }

  .aanmeld-button:hover {
    box-shadow: rgba(253, 76, 0, 0.5) 0 3px 8px;
  }
</style>
