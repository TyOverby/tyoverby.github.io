let max_line_length = 0;
const codeblocks = document.querySelectorAll("pre > code");

for (let block of codeblocks) {
  for (let line of block.innerText.split("\n")) {
    max_line_length = Math.max(max_line_length, line.length);
  }
}

max_line_length = Math.min(max_line_length, 60);

if (max_line_length > 0) {
  for (let block of codeblocks) {
    block.setAttribute("data-chars", max_line_length);
    block.style.setProperty("--code-chars", max_line_length);
  }
}
