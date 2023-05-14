function crearGrafica(resText) {
  var dates = [];
  var bitcoinPrice = [];
  for(var key in resText){
    var date = new Date(key);
    dates.push(date.getDate() + "/" + date.getMonth() + "/" + date.getFullYear());
    bitcoinPrice.push( resText[key] );
  }
  console.log(dates);
  var ctx = document.getElementById("valorBitcoin").getContext('2d');
  var myChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: dates,
      datasets: [{
          label: 'Valor del Bitcoin (EUR)',
          backgroundColor: "rgba(87, 10, 184, 0.3)",
          borderColor: "#570ab8",
          data: bitcoinPrice
      }],
    }
  });
}

function establecerConversion(bitcoin) {
  console.log(bitcoin);
  var valoresEstablecer = document.getElementsByClassName("establecerValorBitcoin");
  for(var key in valoresEstablecer) {
    var euros = valoresEstablecer[key].innerHTML;
    valoresEstablecer[key].innerHTML = (euros/bitcoin).toFixed( 6 );
  }
}

window.onload = function() {
  var bitcoinHistorical = new XMLHttpRequest();
  bitcoinHistorical.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      crearGrafica( JSON.parse(this.responseText).bpi );
    }
  };
  bitcoinHistorical.open("GET", "https://api.coindesk.com/v1/bpi/historical/close.json?index=EUR", true);
  bitcoinHistorical.send();

  var bitcoinPrice = new XMLHttpRequest();
  bitcoinPrice.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      establecerConversion( JSON.parse(this.responseText).bpi.EUR.rate_float );
    }
  };
  bitcoinPrice.open("GET", "https://api.coindesk.com/v1/bpi/currentprice/EUR.json", true);
  bitcoinPrice.send();
};