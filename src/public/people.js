const createButton = document.getElementById("create-person-button");
const personName = document.getElementById("name");
const personAge = document.getElementById("age");

const personId = document.getElementById("person-id");
const deleteButton = document.getElementById("delete-person-button");

async function createPerson(name, age) {
  try {
    const request = await fetch("/create-people", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name: name, age: age })
    });

    if (!request.ok) throw new Error("an error ocurred to request!");
    return await request.text();
  } catch (err) { console.error(err); }
}

async function deletePerson(id) {
  try {
    const request = await fetch("/delete-people", {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ id: id })
    });

    if (!request.ok) throw new Error("an error ocurred to request!");
    return await request.text();
  } catch (err) { console.error(err); }
}

createButton.addEventListener("click", async () => {
  const nameVal = personName.value;
  const ageVal = personAge.value;
  const result = await createPerson(nameVal, ageVal);
  console.log(result);
  window.alert(result);
});

deleteButton.addEventListener("click", async () => {
  const personIdVal = personId.value;
  const result = await deletePerson(Number(personIdVal));
  console.log(result);
  window.alert(result);
});
