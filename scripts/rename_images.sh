for i in ../public/clara/img/*.jpg_large; do
  mv "$i" "${i%.jpg_large}.jpeg"
done