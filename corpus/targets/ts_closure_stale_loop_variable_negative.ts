// SAFE: use let for block-scoped iteration variable
for (let i = 0; i < buttons.length; i++) {
  buttons[i].addEventListener('click', function() {
    console.log('Button ' + i + ' clicked');
  });
}

for (let j = 0; j < items.length; j++) {
  setTimeout(function() {
    processItem(items[j], j);
  }, 1000 * j);
}
