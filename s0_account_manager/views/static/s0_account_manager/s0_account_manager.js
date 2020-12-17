// window.onload = function exampleFunction() {
//   console.log("window.onload");
//   get_all_c1_account();
// };

$(document).ready(function () {
  console.log("ready!");
  get_all_c1_account();
});

function get_all_c1_account() {
  $.get("/get_all_c1_account", function (data) {
    console.log(data);
    $("#list_s0_accounts").empty();
    for (let i = 0; i < data.accounts.length; i++) {
      $("#list_s0_accounts").append(
        `<li class="list-group-item">[${i + 1}]: ${data.accounts[i]}</li>`
      );
    }
  });
}

$("#btn_add_user").click(function () {
  let name = $("#input_add_user_name").val();
  let key = $("#input_add_user_key").val();
  if (
    name === null ||
    key === null ||
    name === undefined ||
    key === undefined ||
    name.length === 0 ||
    key.length === 0
  ) {
    alert("invalid name or key");
    return;
  }

  $.ajax({
    type: "POST",
    url: "/add_c1_account",
    dataType: "json",
    data: JSON.stringify({ name: name, key: key }),
    success: function (data) {
      console.log(data);
      get_all_c1_account();
    },
  });
});

$("#btn_del_user").click(function () {
  let name = $("#input_del_user_name").val();
  if (name === null || name === undefined || name.length === 0) {
    alert("invalid name");
    return;
  }

  $.ajax({
    type: "POST",
    url: "/del_c1_account",
    dataType: "json",
    data: JSON.stringify({ name: name }),
    success: function (data) {
      console.log(data);
      get_all_c1_account();
    },
  });
});
