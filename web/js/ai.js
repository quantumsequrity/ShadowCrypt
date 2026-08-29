'use strict';
/* ShadowCrypt Online — monster & companion AI (real-time ticks) */
var SC = (typeof window !== 'undefined') ? (window.SC = window.SC || {}) : (globalThis.SC = globalThis.SC || {});

SC.ai = (function () {
  var U = SC.util, E = SC.entities, W = SC.worldgen;

  // ctx: { map, player, monsters, allies, occupied(x,y,self), monsterAttackPlayer(m), fireMonsterProjectile(m), moveMonster(m,x,y) }
  function tickMonster(ctx, m, dtMs) {
    if (m.hp <= 0) return;
    // status gates
    for (var i = 0; i < m.effects.length; i++) {
      var fid = m.effects[i].id;
      if (fid === 'stunned' || fid === 'frozen') { return; }
    }
    m.moveCd -= dtMs; m.atkCd -= dtMs;
    var p = ctx.player;
    var invisible = E.hasEffect(p, 'invisible');
    var d = U.dist(m.x, m.y, p.x, p.y);
    var sight = m.boss ? 14 : 9;

    // acquire target
    if (!invisible && d <= sight && hasLos(ctx.map, m.x, m.y, p.x, p.y)) {
      m.aiState = 'chase';
      m.lastSeenPlayer = { x: p.x, y: p.y };
    } else if (m.aiState === 'chase' && (invisible || d > sight * 1.6)) {
      m.aiState = m.lastSeenPlayer ? 'search' : 'idle';
    }

    // pick nearest target among player + allies
    var target = p, targetDist = d, isAllyTarget = false;
    if (ctx.allies) {
      for (var a = 0; a < ctx.allies.length; a++) {
        var al = ctx.allies[a];
        if (al.hp <= 0) continue;
        var ad = U.dist(m.x, m.y, al.x, al.y);
        if (ad < targetDist) { target = al; targetDist = ad; isAllyTarget = true; }
      }
    }

    if (m.aiState === 'chase') {
      // ranged attack
      if (m.ranged && targetDist <= 6 && targetDist > 1.6 && m.atkCd <= 0 && hasLos(ctx.map, m.x, m.y, target.x, target.y)) {
        m.atkCd = attackInterval(m);
        if (ctx.fireMonsterProjectile) ctx.fireMonsterProjectile(m, target);
        return;
      }
      // melee attack
      if (targetDist <= 1.6 && m.atkCd <= 0) {
        m.atkCd = attackInterval(m);
        if (isAllyTarget) { if (ctx.monsterAttackAlly) ctx.monsterAttackAlly(m, target); }
        else if (ctx.monsterAttackPlayer) ctx.monsterAttackPlayer(m);
        return;
      }
      // move toward target
      if (m.moveCd <= 0 && targetDist > 1.0) {
        m.moveCd = moveInterval(m);
        stepToward(ctx, m, target.x, target.y);
      }
    } else if (m.aiState === 'search' && m.lastSeenPlayer) {
      if (m.moveCd <= 0) {
        m.moveCd = moveInterval(m);
        stepToward(ctx, m, m.lastSeenPlayer.x, m.lastSeenPlayer.y);
        if (m.x === m.lastSeenPlayer.x && m.y === m.lastSeenPlayer.y) { m.aiState = 'idle'; m.lastSeenPlayer = null; }
      }
    } else {
      // idle wander
      if (m.moveCd <= 0 && Math.random() < 0.25) {
        m.moveCd = moveInterval(m) * 2;
        var dir = U.DIRS8[Math.floor(Math.random() * 8)];
        tryStep(ctx, m, m.x + dir[0], m.y + dir[1]);
      }
    }
  }

  function attackInterval(m) { return Math.max(600, 2200 - m.spd * 80); }
  function moveInterval(m) { return Math.max(220, 1000 - m.spd * 45); }

  function hasLos(map, x0, y0, x1, y1) {
    var pts = U.line(x0, y0, x1, y1);
    for (var i = 1; i < pts.length - 1; i++) {
      if (map.blocksSight(pts[i][0], pts[i][1])) return false;
    }
    return true;
  }

  function stepToward(ctx, m, tx, ty) {
    // direct step first
    var dx = Math.sign(tx - m.x), dy = Math.sign(ty - m.y);
    if (tryStep(ctx, m, m.x + dx, m.y + dy)) return true;
    if (dx && tryStep(ctx, m, m.x + dx, m.y)) return true;
    if (dy && tryStep(ctx, m, m.x, m.y + dy)) return true;
    // fall back to a short BFS path (bosses & smarter chasers)
    var path = W.findPath(ctx.map, m.x, m.y, tx, ty, 400, function (x, y) { return ctx.occupied(x, y, m); });
    if (path && path.length) return tryStep(ctx, m, path[0].x, path[0].y);
    return false;
  }

  function tryStep(ctx, m, nx, ny) {
    var map = ctx.map;
    if (!map.isWalkable(nx, ny)) {
      // monsters can open doors occasionally (not bosses' gates)
      if (map.get(nx, ny) === SC.TILE.DOOR_CLOSED && !m.boss && Math.random() < 0.4) {
        map.set(nx, ny, SC.TILE.DOOR_OPEN);
      }
      return false;
    }
    if (map.get(nx, ny) === SC.TILE.LAVA) return false; // monsters avoid lava
    if (ctx.occupied(nx, ny, m)) return false;
    m.x = nx; m.y = ny;
    return true;
  }

  // Companion / summoned ally AI: follow player, attack nearby monsters
  function tickAlly(ctx, al, dtMs) {
    if (al.hp <= 0) return;
    al.moveCd = (al.moveCd || 0) - dtMs;
    al.atkCd = (al.atkCd || 0) - dtMs;
    var p = ctx.player;
    // nearest living monster within range
    var best = null, bd = 1e9;
    for (var i = 0; i < ctx.monsters.length; i++) {
      var m = ctx.monsters[i];
      if (m.hp <= 0) continue;
      var d = U.dist(al.x, al.y, m.x, m.y);
      if (d < bd && d <= 6) { bd = d; best = m; }
    }
    if (best) {
      if (bd <= 1.6 && al.atkCd <= 0) {
        al.atkCd = 1100;
        var dmg = SC.combat.computeDamage(al.atk, best.def, 1);
        best.hp -= dmg;
        best.aiState = 'chase';
        if (ctx.spawnFloatText) ctx.spawnFloatText(best.x, best.y, '-' + dmg, '#9be8ff');
        if (best.hp <= 0 && ctx.onMonsterKilled) ctx.onMonsterKilled(best, true);
        return;
      }
      if (al.moveCd <= 0 && bd > 1.0) {
        al.moveCd = 300;
        stepAlly(ctx, al, best.x, best.y);
      }
      return;
    }
    // follow player
    var pd = U.dist(al.x, al.y, p.x, p.y);
    if (pd > 2.2 && al.moveCd <= 0) {
      al.moveCd = 260;
      stepAlly(ctx, al, p.x, p.y);
    }
    if (pd > 12) { al.x = p.x; al.y = p.y; al.fx = p.x; al.fy = p.y; } // teleport if left behind
  }

  function stepAlly(ctx, al, tx, ty) {
    var dx = Math.sign(tx - al.x), dy = Math.sign(ty - al.y);
    var cand = [[al.x + dx, al.y + dy], [al.x + dx, al.y], [al.x, al.y + dy]];
    for (var i = 0; i < cand.length; i++) {
      var nx = cand[i][0], ny = cand[i][1];
      if (ctx.map.isWalkable(nx, ny) && ctx.map.get(nx, ny) !== SC.TILE.LAVA && !ctx.occupied(nx, ny, al)) {
        al.x = nx; al.y = ny;
        return true;
      }
    }
    return false;
  }

  return { tickMonster: tickMonster, tickAlly: tickAlly, hasLos: hasLos };
})();
