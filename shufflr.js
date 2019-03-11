export function shuffle(list, seed) {
  var shuffleItem, i, j;

  if (!Array.isArray(list)) {
    return [];
  }

  if (list.length <= 2) {
    return list;
  }

  seed = seed || 10000;

  for (i = 0; i < list.length - 2; i++) {
    j = (Math.round(Math.random() * seed) + i) % list.length;

    shuffleItem = list[i];
    list[i] = list[j];
    list[j] = shuffleItem;
  }

  return list;
}
