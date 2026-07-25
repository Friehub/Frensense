// [frensense]
// observation: var-declared loop variable captured by async callback or closure. All iterations share the same binding, which has already changed by the time the callback runs.
// impact: Every callback sees the final value of i (array.length) instead of the value at the time of iteration. This causes silent logic bugs where actions are applied to the wrong element or index is out of bounds.
// improvement: Use let instead of var, or create a closure per iteration with an IIFE.
// cwe: CWE-829
// cvss: 5.3
// owasp: 
// severity: Medium

for (var i = 0; i < buttons.length; i++) {
  // VULNERABLE: when clicked, i is always buttons.length
  buttons[i].addEventListener('click', function() {
    console.log('Button ' + i + ' clicked');
  });
}

for (var j = 0; j < items.length; j++) {
  // VULNERABLE: setTimeout uses final j
  setTimeout(function() {
    processItem(items[j], j);
  }, 1000 * j);
}
