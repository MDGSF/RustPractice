"use strict";

$(document).ready(function () {
  console.log("ready");
  get_all_c1_account();
});

function get_all_c1_account() {
  $.get("/get_all_c1_account", function (data) {
    console.log(data);

    var div_s0_accounts_number = document.getElementById('s0_accounts_number');
    div_s0_accounts_number.innerHTML = `当前用户数量：${data.numbers}`;

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
    show_alert_msg("invalid name or key");
    return;
  }

  $.ajax({
    type: "POST",
    url: "/add_c1_account",
    dataType: "json",
    data: JSON.stringify({ name: name, key: key }),
    success: function (data) {
      show_alert_msg(data.message);
      get_all_c1_account();
    },
  });
});

$("#btn_del_user").click(function () {
  let name = $("#input_del_user_name").val();
  if (name === null || name === undefined || name.length === 0) {
    show_alert_msg("invalid name");
    return;
  }

  $.ajax({
    type: "POST",
    url: "/del_c1_account",
    dataType: "json",
    data: JSON.stringify({ name: name }),
    success: function (data) {
      show_alert_msg(data.message);
      get_all_c1_account();
    },
  });
});

function show_alert_msg(msg) {
  $("#alert-wrapper").empty();
  $("#alert-wrapper").append(`
<div class="alert alert-success alert-dismissible fade show" role="alert">
  ${msg}
  <button type="button" class="close" data-dismiss="alert" aria-label="Close">
    <span aria-hidden="true">&times;</span>
  </button>
</div>`);
}
