<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { exit } from "@tauri-apps/api/process";
  import { invoke } from "@tauri-apps/api/tauri";

  let time = new Date().getTime();
  let intervalTime = 0;
  /**
   * @type {any[]}
   */
  let holidays = [];

  function formatCurrentTimeAndDate() {
    const now = new Date();

    const hours = now.getHours().toString().padStart(2, "0");
    const minutes = now.getMinutes().toString().padStart(2, "0");
    const day = now.getDate().toString().padStart(2, "0");
    const month = now.getMonth(); // Note: Month is 0-based, so we add 1.
    const year = now.getFullYear();
    const monthNames = [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sep",
      "Oct",
      "Nov",
      "Dec",
    ];
    const formattedMonth = monthNames[month];

    const formattedDateTime = ` ${hours}:${minutes}u ${day} ${formattedMonth} ${year}`;

    return formattedDateTime;
  }

  /**
   * @param {string} time
   */
  function formateDate(time) {
    const day = time.substr(8, 2); // Extract the day part
    const month = time.substr(5, 2); // Extract the month part

    // Convert the month part to the first 3 letters of the month
    const monthNames = [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sep",
      "Oct",
      "Nov",
      "Dec",
    ];
    const formattedMonth = monthNames[parseInt(month, 10) - 1];

    return `${day} ${formattedMonth}`;
  }

  let formattedTimeDate = formatCurrentTimeAndDate();

  // checks connection
  onMount(() => {
    invoke("get_hollidays")
      .then((e) => {
        holidays = [];
        let holidaysJSON = JSON.parse(e);
        for (let i = 0; i < holidaysJSON.length; i++) {
          holidays.push({
            fromDate: formateDate(holidaysJSON[i].date),
            toDate: formateDate(holidaysJSON[i].toDate),
            reason: holidaysJSON[i].reason,
          });
        }
      })
      .catch((err) => console.log(err));

    listen("ping", (event) => {
      time = new Date().getTime();
      intervalTime = 0;
    });

    setInterval(() => {
      formattedTimeDate = formatCurrentTimeAndDate();
    }, 10000);
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

  <div class="column2">
    <div style="font-size: 38px; font-weight: 200;">
      <span>{formattedTimeDate}</span>
    </div>
    <div class="Openingsurenbox" style="margin-top: 20px;">
      <span class="title">Pauzes / Breaks</span>
      <div class="daytime">
        <div class="time" style="margin-left: 0; text-align: left;">
          <span>10:00 - 10:20</span>
          <span>12:30 - 13:00</span>
        </div>
      </div>
    </div>
    <div class="Openingsurenbox" style="margin-top: 40px;">
      <span class="title">Openingsuren / Openingtimes</span>
      <div class="daytime">
        <div class="day">
          <span>Ma / Mon:</span>
          <span>Di / Tue:</span>
          <span>Wo / Wen:</span>
          <span>Do / Thu:</span>
          <span>Vr / Fri:</span>
          <span>Za / Sat:</span>
          <span>Zo / Sun:</span>
        </div>
        <div class="time">
          <span>7:00 - 15:45</span>
          <span>7:00 - 15:45</span>
          <span>7:00 - 15:45</span>
          <span>7:00 - 15:45</span>
          <span>7:00 - 15:00</span>
          <span>Gesloten / Closed</span>
          <span>Gesloten / Closed</span>
        </div>
      </div>
    </div>

    <div class="Openingsurenbox" style="margin-top: 40px;">
      <span class="title">Vakanties / Hollidays</span>
      <div class="daytime">
        <div class="day">
          {#each holidays as holiday, key (key)}
            <span
              >{holiday.fromDate}
              {holiday.fromDate === holiday.toDate
                ? ""
                : `- ${holiday.toDate}`}</span
            >
          {/each}
        </div>
        <div class="time">
          {#each holidays as holiday, key (key)}
            <span>{holiday.reason}</span>
          {/each}
        </div>
      </div>
    </div>
  </div>
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

    display: flex;
  }

  .column2 {
    min-width: 340px;
    max-width: 340px;
    margin-top: 30px;
  }

  .Openingsurenbox {
    display: flex;
    flex-direction: column;
  }

  .title {
    font-size: 20px;
    font-weight: 400;
    font-family: Inter, -apple-system, system-ui, Roboto, "Helvetica Neue",
      Arial, sans-serif;
    margin-bottom: 20px;
    background-color: initial;
    background-image: linear-gradient(-180deg, #c82c3e, #a02c3f);
    color: white;
    padding: 10px;
    border-radius: 5px;
  }

  .daytime {
    display: flex;
  }

  .day {
    display: flex;
    flex-direction: column;
    font-size: 18px;
    font-weight: 400;
  }

  .time {
    display: flex;
    flex-direction: column;
    margin-left: auto;
    text-align: right;
    font-size: 16px;
  }

  .aanmeld-row {
    padding-top: 20px;
    padding-left: 5px;
  }

  .language-row {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
  }

  .language-row img {
    height: 96px;
    width: 96px;
    margin-right: 5px;
  }

  .language-text {
    text-align: left;
    line-height: 0px;
  }

  .language-text h2 {
    font-size: 30px;
  }

  .aanmeld-button {
    font-size: 20px;
    font-weight: 600;
    font-family: Inter, -apple-system, system-ui, Roboto, "Helvetica Neue",
      Arial, sans-serif;
    background-color: initial;
    background-image: linear-gradient(-180deg, #c82c3e, #a02c3f);
    border-radius: 6px;
    box-shadow: rgba(0, 0, 0, 0.1) 0 2px 4px;
    color: #ffffff;
    cursor: pointer;
    display: inline-block;
    height: 45px;
    line-height: 45px;
    outline: 0;
    overflow: hidden;
    padding: 0 10px;
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
