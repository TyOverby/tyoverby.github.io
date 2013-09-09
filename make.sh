cpp -w resume.phtml | sed '/^#/ d' > ./resume.html
cpp -w code.phtml | sed '/^#/ d' > ./code.html
cpp -w index.phtml | sed '/^#/ d' > ./index.html
