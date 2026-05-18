const expenseForm = document.getElementById("expenseForm");
const expenseList = document.getElementById("expenseList");

const totalExpense = document.getElementById("totalExpense");
const transactionCount = document.getElementById("transactionCount");
const summaryText = document.getElementById("summaryText");

let expenses = [];

expenseForm.addEventListener("submit", function (e) {
  e.preventDefault();

  const itemName = document.getElementById("itemName").value;
  const amount = document.getElementById("amount").value;
  const category = document.getElementById("category").value;
  const note = document.getElementById("note").value;

  if (!itemName || !amount) {
    alert("Please fill all required fields.");
    return;
  }

  const expense = {
    id: Date.now(),
    itemName,
    amount: Number(amount),
    category,
    note,
  };

  expenses.push(expense);

  renderExpenses();

  expenseForm.reset();
});

function renderExpenses() {
  expenseList.innerHTML = "";

  if (expenses.length === 0) {
    expenseList.innerHTML = `
      <p class="empty">
        No expense data yet.
      </p>
    `;
  }

  let total = 0;

  expenses.forEach((expense) => {
    total += expense.amount;

    expenseList.innerHTML += `
      <div class="expense-card">

        <div>
          <h3>${expense.itemName}</h3>

          <p>
            Rp ${expense.amount.toLocaleString("id-ID")}
            •
            ${expense.category}
          </p>

          <small>${expense.note}</small>
        </div>

        <button
          class="delete-btn"
          onclick="deleteExpense(${expense.id})"
        >
          Delete
        </button>

      </div>
    `;
  });

  totalExpense.textContent =
    `Rp ${total.toLocaleString("id-ID")}`;

  transactionCount.textContent = expenses.length;

  if (total === 0) {
    summaryText.textContent = "No expenses recorded yet";
  } else if (total < 100000) {
    summaryText.textContent = "Low spending activity";
  } else if (total < 500000) {
    summaryText.textContent = "Moderate spending activity";
  } else {
    summaryText.textContent = "High spending activity";
  }
}

function deleteExpense(id) {
  expenses = expenses.filter(
    (expense) => expense.id !== id
  );

  renderExpenses();
}