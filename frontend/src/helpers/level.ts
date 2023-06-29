function getXPTillNextLevel(level: number, currentXP: number): number {
    return Math.max(5 * Math.pow(level, 2) + 50 * level + 100 - currentXP, 0);
}

function getXPForLevel(level: number): number {
    if (level <= 0) return 0;

    let xp = 0;
    for (let i = 0; i < level; i++) {
        xp += getXPTillNextLevel(i, 0);
    }
    return xp;
}

export function getLevelProgress(level: number, currentXP: number): number {
    const xpProgress = currentXP - getXPForLevel(level);
    const xpNeeded = getXPForLevel(level + 1) - getXPForLevel(level);

    return Math.min(xpProgress / xpNeeded, 1);
}
