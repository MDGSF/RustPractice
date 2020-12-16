window.onload = function exampleFunction() {
  get_all_c1_account();
};

function get_all_c1_account() {
  $.get("/get_all_c1_account", function (data) {
    console.log(data);
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
  $.ajax({
    type: "POST",
    url: "/add_c1_account",
    dataType: "json",
    data: JSON.stringify({ name: name, key: key }),
    success: function (data) {
      console.log(data);
    },
  });
});

$("#btn_del_user").click(function () {
  let name = $("#input_del_user_name").val();
  $.ajax({
    type: "POST",
    url: "/del_c1_account",
    dataType: "json",
    data: JSON.stringify({ name: name }),
    success: function (data) {
      console.log(data);
    },
  });
});
