export function shuffle(list, seed) {
  var shuffleItem, i, j, shuffledList;

  if (!Array.isArray(list)) {
    return [];
  }

  shuffledList = Array.from(list);

  if (shuffledList.length <= 2) {
    return list;
  }

  seed = seed || 10000;

  for (i = 0; i < shuffledList.length - 2; i++) {
    j = (Math.round(Math.random() * seed) + i) % shuffledList.length;

    shuffleItem = shuffledList[i];
    shuffledList[i] = list[j];
    shuffledList[j] = shuffleItem;
  }

  return shuffledList;
}
