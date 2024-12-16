export function createBalanceBadge(role, stars) {
    return {
        schemaVersion: 1,
        label: `${role} balance`,
        message: `${stars} STARS`,
        color: stars === '0.000000' ? 'red' : 'green',
        style: 'flat-square'
    };
}

export function createTotalBalanceBadge(totalStars) {
    return {
        schemaVersion: 1,
        label: 'total balance',
        message: `${totalStars.toFixed(6)} STARS`,
        color: totalStars > 0 ? 'blue' : 'red',
        style: 'flat-square'
    };
}

export function createBadgeFiles(balances) {
    const files = {};
    
    // Individual balance badges
    balances.forEach(({ role, stars }) => {
        files[`${role}-balance.json`] = {
            content: JSON.stringify(createBalanceBadge(role, stars))
        };
    });

    // Total balance badge
    const totalStars = balances.reduce((sum, { stars }) => sum + parseFloat(stars), 0);
    files['total-balance.json'] = {
        content: JSON.stringify(createTotalBalanceBadge(totalStars))
    };

    return files;
} 