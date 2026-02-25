const selected_operator = document.getElementsByClassName("select-operator");
const number1 = document.getElementById("number1");
const number2 = document.getElementById("number2");
const runButton = document.getElementById("run");

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

    if (request.ok) {
      const successResponse = request.text();
      return successResponse;
    }
  } catch (err) { console.error(err); }
};

document.addEventListener("DOMContentLoaded", () => {
  runButton.addEventListener("click", async () => {
    const n1 = number1.value;
    const n2 = number2.value;

    switch (selected_operator.value.trim()) {
      case "sum":
        let result = await post_request("sum", n1, n2);
        window.alert(result);
    }
  });
});
