<script lang="ts">
  import Loader from "$lib/Loader.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let topic = "";
  let message = "";
  let name = "";
  $: iemandNodig = true;

  let updatedTime = "";
  $: time = new Date().toLocaleTimeString();

  onMount(() => {
    const interval = setInterval(() => {
      time = new Date().toLocaleTimeString();
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  });

  const getName = () => {
    let keuzeMenu = document.getElementById("keuze-menu");
    if (keuzeMenu != null) {
      keuzeMenu.style.display = "none";
    }

    let getNameMenu = document.getElementById("get-name-menu");
    if (getNameMenu != null) {
      getNameMenu.style.display = "flex";
    }
  };

  const sendNotification = () => {
    let getNameMenu = document.getElementById("get-name-menu");
    if (getNameMenu != null) {
      getNameMenu.style.display = "none";
    }

    if (iemandNodig) {
      let statusMenu = document.getElementById("status-menu");
      if (statusMenu != null) {
        statusMenu.style.display = "flex";
      }

      updatedTime = time;

      // send alert accepted
      invoke("send_message_to_own_topic", {
        datapoint: topic,
        value: `${message} ${name}.`,
      })
        .then((_e) => {
          let melding1 = document.getElementById("melding1");
          if (melding1 != null) {
            melding1.style.display = "none";
          }

          updatedTime = time;

          let melding = document.getElementById("melding2");
          if (melding != null) {
            melding.style.display = "block";
          }

          listen(topic, (event) => {
            if (event.payload === `${message} ${name}./ alert accepted`) {
              let melding2 = document.getElementById("melding2");
              if (melding2 != null) {
                melding2.style.display = "none";
              }

              updatedTime = time;

              let onderweg = document.getElementById("onderweg");
              if (onderweg != null) {
                onderweg.style.display = "flex";
              }
            }
          });
        })
        .catch((e) => console.log(e));
    } else {
      let statusMenu = document.getElementById("afgeef-menu");
      if (statusMenu != null) {
        statusMenu.style.display = "flex";
      }

      // send alert accepted
      invoke("send_message_to_own_topic", {
        datapoint: topic,
        value: `${message} ${name}.`,
      })
        .then((e) => console.log(e))
        .catch((e) => console.log(e));
    }
  };

  const goodbyeScreen = () => {
    let getNameMenu = document.getElementById("afgeef-menu");
    if (getNameMenu != null) {
      getNameMenu.style.display = "none";
    }

    let getStatusMenu = document.getElementById("status-menu");
    if (getStatusMenu != null) {
      getStatusMenu.style.display = "none";
    }

    let afscheidMenu = document.getElementById("afscheid-menu");
    if (afscheidMenu != null) {
      afscheidMenu.style.display = "flex";
    }

    setTimeout(() => {
      let afscheidMenu = document.getElementById("afscheid-menu");
      if (afscheidMenu != null) {
        afscheidMenu.style.display = "none";
      }

      window.location.href = "/";
    }, 5000);
  };
</script>

<main class="main">
  <div class="header">
    <a href="/">
      <img src="/icons8-france-96.png" alt="France" />
    </a>
    <a href="/">
      <div class="home">
        <svg
          fill="#2a2a2a"
          width="100px"
          height="100px"
          viewBox="0 0 32 32"
          xmlns="http://www.w3.org/2000/svg"
          ><path
            d="M27 18.039L16 9.501 5 18.039V14.56l11-8.54 11 8.538v3.481zm-2.75-.31v8.251h-5.5v-5.5h-5.5v5.5h-5.5v-8.25L16 11.543l8.25 6.186z"
          /></svg
        >
      </div>
    </a>
  </div>

  <div class="time">
    <h4>{time}</h4>
  </div>

  <div class="keuze-menu" id="keuze-menu">
    <h1>Choisissez l'une des options ci-dessous</h1>

    <button
      on:click={() => {
        topic = "Heftruck-chauffeur";
        message = "Heftruckchauffeur gevraagd aan de poort: ";
        getName();
      }}>Je viens décharger et j'ai besoin d'un cariste.</button
    >

    <button
      on:click={() => {
        topic = "Hulp";
        message = "Komt iets ophalen: ";
        getName();
      }}>Je suis ici pour récupérer quelque chose.</button
    >

    <button
      on:click={() => {
        topic = "Hulp";
        message = "Komt iets brengen en moet getekend worden: ";
        getName();
      }}>Je dépose un colis et j'ai besoin que quelqu'un signe.</button
    >

    <button
      on:click={() => {
        iemandNodig = false;
        topic = "Gebracht";
        message = "Heeft iets gebracht: ";
        getName();
      }}>Je dépose un colis et je n'ai besoin de personne.</button
    >

    <button
      on:click={() => {
        topic = "Leiding gevende";
        message = "Leiding gevende gevraagd aan de poort: ";
        getName();
      }}>J'aimerais parler au directeur.</button
    >
  </div>

  <div class="get-name-menu" id="get-name-menu">
    <h4>Entrez le nom de votre entreprise ou votre nom personnel.</h4>
    <div>
      <input id="bedrijfsnaam-input" placeholder="Nom" bind:value={name} />
      <button on:click={sendNotification}>Suivante</button>
    </div>
    <div class="quick-buttons-div">
      <button
        on:click={() => {
          name = "MDL";
          sendNotification();
        }}>MDL</button
      >
      <button
        on:click={() => {
          name = "UPS";
          sendNotification();
        }}>UPS</button
      >
      <button
        on:click={() => {
          name = "Titgemeyer";
          sendNotification();
        }}>Titgemeyer</button
      >
      <button
      on:click={() => {
        name = "Claasen";
        sendNotification();
      }}>Claassen</button
    >
      <button
        on:click={() => {
          name = "MCB";
          sendNotification();
        }}>MCB</button
      >
      <button
        on:click={() => {
          name = "Mayfran";
          sendNotification();
        }}>Mayfran</button
      >
    </div>
  </div>

  <div class="status-menu" id="status-menu">
    <div style="display: flex; flex-direction: column; align-items: center;">
      <div style="width: 300px;">
        <Loader />
      </div>

      <h3 style="color: black; display: block;" id="melding1">
        <span>{updatedTime}</span> Envoyer une notification... . Patience.
      </h3>

      <h3 style="color: black; display: none" id="melding2">
        <span>{updatedTime}</span> Notification envoyée. Patience.
      </h3>

      <div
        id="onderweg"
        style="display: none; align-items: center; flex-direction: column;"
      >
        <h3 style="color: black; margin-bottom: 20px">
          <span>{updatedTime}</span> Quelqu'un est en route! Patience.
        </h3>

        <button on:click={goodbyeScreen}
          >Cliquez ici si vous avez été aidé.</button
        >
      </div>
    </div>
  </div>

  <div class="afgeef-menu" id="afgeef-menu">
    <div style="display: flex; flex-direction: column; align-items: center;">
      <h4>Veuillez laisser votre colis ici.</h4>
      <img src="/pakketje.PNG" alt="pakketje" />
      <button on:click={goodbyeScreen}
        >Cliquez ici une fois que vous avez placé le colis.</button
      >
    </div>
  </div>

  <div class="afscheid-menu" id="afscheid-menu">
    <div
      style="display: flex; flex-direction: column; align-items: center; text-align: center;"
    >
      <h4>Merci! Au nom de toute l'équipe Esma, bonne journée !</h4>
    </div>
  </div>
</main>

<style>
  .main {
    margin: 0;
    padding: 0;
    position: relative;

    background-image: url("/Background.svg");

    /* Full height */
    height: 100vh;
    width: 100vw;

    /* Center and scale the image nicely */
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
  }

  .header {
    display: flex;
    align-items: center;
    position: absolute;
    top: 5px;
    left: 10px;
  }

  .time {
    position: absolute;
    top: -25px;
    right: 15px;
  }

  .time h4 {
    font-size: 45px;
  }

  .keuze-menu {
    display: block;
    height: 100vh;
    width: 100vw;
    display: flex;
    align-items: center;
    flex-direction: column;
  }

  .keuze-menu h1 {
    margin-top: 100px;
    font-size: 45px;
  }

  .keuze-menu button {
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
    height: 80px;
    line-height: 80px;
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
    margin-top: 30px;
  }

  .keuze-menu button:hover {
    box-shadow: rgba(253, 76, 0, 0.5) 0 3px 8px;
  }

  .get-name-menu button {
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
    height: 60px;
    line-height: 60px;
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

  .get-name-menu button:hover {
    box-shadow: rgba(253, 76, 0, 0.5) 0 3px 8px;
  }

  #bedrijfsnaam-input {
    border: none;
    border-radius: 20px;
    height: 60px;
    line-height: 60px;
    font-size: 26px;
    padding-left: 15px;
    margin-right: 20px;
    width: 600px;
  }

  .get-name-menu h4 {
    font-size: 35px;
    margin-top: 200px;
  }

  .get-name-menu {
    display: none;
    height: 100vh;
    width: 100vw;
    flex-direction: column;
    align-items: center;
  }

  .get-name-menu div {
    display: flex;
    align-items: center;
  }

  .status-menu {
    display: none;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }

  .status-menu h3 {
    font-size: 45px;
  }

  .afgeef-menu {
    display: none;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }

  .afgeef-menu img {
    height: 400px;
  }

  .afgeef-menu h4 {
    font-size: 30px;
  }

  .afgeef-menu button {
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
    height: 60px;
    line-height: 60px;
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
    margin-top: 30px;
  }

  .afscheid-menu {
    display: none;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }

  .afscheid-menu h4 {
    font-size: 40px;
    width: 900px;
    line-height: 60px;
  }

  #onderweg button {
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
    height: 60px;
    line-height: 60px;
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
    margin-top: 30px;
  }

  button:hover {
    box-shadow: rgba(253, 76, 0, 0.5) 0 3px 8px;
  }
</style>
