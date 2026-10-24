/**
 * Voices by temperament, appended to the prompt's opening sentence. They
 * describe only the voice: the character's personality belongs to the
 * description in each world.
 */
export const voicePresets: readonly { name: string; description: string }[] = [
  { name: '开朗', description: '声音明亮、有活力，语调轻快上扬，语速稍快。' },
  { name: '活泼', description: '声音清亮跳跃，语调起伏大，语速快，说话时带着笑意。' },
  { name: '温柔', description: '声音柔和温暖，气息轻，语速舒缓，语气亲切。' },
  { name: '沉稳', description: '声音低沉平稳，吐字清晰，语速偏慢，语气从容。' },
  { name: '知性', description: '声音清晰干净，语调平稳，吐字准确，语速适中，语气理性。' },
  { name: '冷淡', description: '声音清冷，语调平直、起伏小，语速平稳，语气疏离。' },
  { name: '腼腆', description: '声音细小，略带犹豫，语速偏慢，句尾声音变轻。' },
  { name: '慵懒', description: '声音松弛，略带沙哑，语速慢，句尾拖长。' },
  { name: '威严', description: '声音浑厚有力，语调坚定，语速偏慢，不怒自威。' },
  { name: '慈祥', description: '声音温和，略带沙哑，语速慢，语气和蔼。' },
]
