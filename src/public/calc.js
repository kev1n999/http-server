const button = document.getElementById("run");
const number1 = document.getElementById("number1");
const number2 = document.getElementById("number2");
const selected_operator = document.getElementById("select-operator");

async function post_request(operation, number1, number2) {
  try {
    const request = await fetch("/calculator", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        operation: operation,
          number1: number1,
          number2: number2,
        })
      });

    if (!request.ok) throw new Error("an error ocurred to request!");
      return await request.text();
  } catch (err) { console.error(err); }
};

button.addEventListener("click", async (e) => {
  e.preventDefault();
  const n1 = Number(number1.value);
  const n2 = Number(number2.value);
  let result;

  switch (selected_operator.value.trim()) {
    case "sum":
      result = await post_request("sum", n1, n2);
      window.alert(result);
      break;

    case "sub":
      result = await post_request("sub", n1, n2);
      window.alert(result);
      break;

    case "mult":
      result = await post_request("mult", n1, n2);
      window.alert(result);
      break;

    case "div":
      result = await post_request("div", n1, n2);
      window.alert(result);
      break;
  }
});
