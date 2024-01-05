<script lang="ts">
let formElement;
  
let conversion: string[] = [];
  const convert = async (e: any) => {
    e.preventDefault();

const formData = new FormData(formElement!)
    let contents = formData.get("textarea")!

    const dev = "http://127.0.0.1:8081"
    const deployment = "https://to-css-var.fly.dev"
    const data = await fetch(deployment+ "/convert", {
      method: "POST",
      body: JSON.stringify({message: contents})
    })

    const message = await data.json()

    console.log(message)

    conversion = message.message.split("\n")

    console.log(conversion)

    navigator.clipboard.writeText( conversion.join("\n") )
  }

</script>

<main>

  <h1>Convert SUI to CSS Variables</h1>
  <div class="flex">

    <form bind:this={formElement} on:submit={convert}>
      <textarea name="textarea"></textarea>
      
      <button type="submit">Convert</button>
    </form>

    <div class="display">
      <p>
        {#each conversion as line}
        {line}<br/>
        {/each}
      </p>
    </div>
  </div>
</main>

<style>
  textarea {
    width: 100%;
    height: 100%;
  }


  h1 {
    color: var(--gray98);
  }
  p {
    color: var(--gray92);
  }
  .display {
    flex: 1;
    width: 100%;
    background-color: var(--gray25);
  }
  form {
    flex: 1;
    background-color: green;
  
  }

  textarea {
    color: var(--gray92);
    background-color: var(--gray30);
    border: none;
    resize: none;
  }
  
  h1 {
    margin-top: 2.4rem;
    margin-bottom: 3.6rem;
    text-align: center;
  }
 * {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
 }

  .flex {
    display: flex;
    width: 100%;
    height: 50vh;
    padding: 0 2.4rem;
  }

  button {
    position: absolute;
    top: 70%;
    left: 50%;
    transform: translate(-50%, -50%);
    cursor: pointer;
    padding: 1.2rem 2.4rem;
    /* background-color: var(); */
  }

</style>