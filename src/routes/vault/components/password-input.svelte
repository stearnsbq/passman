<script lang="ts">
  import zxcvbn from "zxcvbn";
  import { invoke } from "@tauri-apps/api/tauri";

  let meter = 0;
  export let password = "";
  let type = "password";
  let showPassword = false;
  export let showFeedback = false;
  let feedback = [];

  const meterClasses = ["weak", "average", "good", "strong"];

  let meterClass = "weak";

  function onPasswordInput(event) {
    password = event.target.value;

    const passwordResult = zxcvbn(password);

    feedback = passwordResult.feedback.suggestions;

    meter = passwordResult.score;

    meterClass = meterClasses[meter - 1];
  }

  function onTogglePassword() {
    showPassword = !showPassword;

    type = showPassword ? "text" : "password";
  }
</script>

<div class="password-input-content">
  <div class="password-input">
    <input
      maxlength="20"
      placeholder="Password"
      value={password}
      {type}
      on:input={onPasswordInput}
    />
    <span on:click={onTogglePassword}>{showPassword ? "Show" : "Hide"}</span>
  </div>

  {#if password.length}
    <meter
      class={meterClass}
      value={meter}
      low="1"
      max="4"
      id="password-strength-meter"
    />
    <span>
      {#if meter === 4}
        Strong
      {:else if meter === 3}
        Good
      {:else if meter === 2}
        Average
      {:else}
        Weak
      {/if}
    </span>
  {/if}

  {#if showFeedback}
    <div class="feedback-list">
      <h4>Feedback</h4>

      <ul>
        {#each feedback as feed}
          <li>{feed}</li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style lang="scss">
  .feedback-list {
    color: white;
  }

  .password-input-content {
    display: flex;
    flex-direction: column;

    .weak {
      &::-webkit-meter-optimum-value {
        background: red;
      }
    }

    .average {
      &::-webkit-meter-optimum-value {
        background: orange;
      }
    }

    .good {
      &::-webkit-meter-optimum-value {
        background: green;
      }
    }

    .strong {
      &::-webkit-meter-optimum-value {
        background: green;
      }
    }

    meter {
      width: 100%;
    }

    span {
      text-align: center;
      color: white;
    }
  }

  .password-input {
    position: relative;
    width: 100%;
    input {
      width: 100%;
    }
    span {
      user-select: none;
      cursor: pointer;
      position: absolute;
      color: #3b3c36;
      font-size: 16px;
      font-weight: 600;
      top: 5px;
      right: 5px;
    }
  }
</style>
