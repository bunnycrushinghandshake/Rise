const baseValues: Record<number, number> = {
  "-1": 1,
  "0": 1,
  "1": 2,
  "2": 3,
  "3": 4,
  "4": 5,
  "5": 6,
  "6": 7,
  "7": 8,
  "8": 10,
  "9": 12,
  "10": 14,
  "11": 17,
  "12": 20,
  "13": 23,
  "14": 26,
  "15": 29,
  "16": 32,
  "17": 36,
  "18": 40,
  "19": 45,
  "20": 50,
  "21": 55,
  "22": 60,
  "23": 65,
  "24": 70,
  "25": 75,
};

export function damageResistanceByLevel(level: number, constitution: number | null) {
  return baseValues[Math.max(level, constitution || 0)];
}
