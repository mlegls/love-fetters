cd ../public/clara/img/
for i in *.jpg_large; do
  mv "$i" "${i%.jpg_large}.jpeg"
done