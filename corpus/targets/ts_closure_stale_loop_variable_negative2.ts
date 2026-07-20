// SAFE alternative: IIFE captures current value
for (var i = 0; i < buttons.length; i++) {
  (function(idx) {
    buttons[idx].addEventListener('click', function() {
      console.log('Button ' + idx + ' clicked');
    });
  })(i);
}

for (var j = 0; j < items.length; j++) {
  (function(idx) {
    setTimeout(function() {
      processItem(items[idx], idx);
    }, 1000 * idx);
  })(j);
}
