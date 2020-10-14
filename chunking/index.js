const regularTable = document.createElement("regular-table");
document.body.appendChild(regularTable);

const IDLE = 0
const LOCKED = 1
const PROCESSING = 2
const DONE = 3
const LOOKING = 4

function defaultCell() {
    return { state: IDLE, progress: 5 }
}

var num_rows = 0;
const DATA = [];

const MAX_WIDTH = 32;
const MAX_HEIGHT = 32;

for (var x = 0; x < MAX_WIDTH; x++) {
    DATA.push([])
    for (var y = 0; y < MAX_HEIGHT; y++) {
        DATA[x][y] = defaultCell()
    }
}

function formatCell(data) {
    return data.progress
}

function dataListener(x0, y0, x1, y1) {
    return {
        num_rows: (num_rows = DATA[0].length),
        num_columns: DATA.length,
        data: DATA.slice(x0, x1).map((col) => col.slice(y0, y1).map(formatCell)),
    };
}

function styleListener() {
    for (const td of regularTable.querySelectorAll("td")) {
        const meta = regularTable.getMeta(td);
        const val = DATA[meta.x][meta.y];
        td.className = "";
        td.classList.toggle("idle", val.state == IDLE);
        td.classList.toggle("locked", val.state == LOCKED);
        td.classList.toggle("processing", val.state == PROCESSING);
        td.classList.toggle("done", val.state == DONE);
    }
}

const NUM_PROCESSORS = 256;

var threadLocalStorage = []
for (var i = 0; i < NUM_PROCESSORS; i++) {
    threadLocalStorage.push({ x: 0, y: 0, state: LOOKING, done: false })
}

function acquireCellLocks(x, y) {
    if(!DATA[x][y]) {
        return false
    }
    if(DATA[x][y].state != IDLE) {
        return false
    }

    if(DATA[x + 1] && DATA[x + 1][y] && (DATA[x + 1][y].state == LOCKED || DATA[x + 1][y].state == PROCESSING)) {
        return false
    }
    if(DATA[x - 1] && DATA[x - 1][y] && (DATA[x - 1][y].state == LOCKED || DATA[x - 1][y].state == PROCESSING)) {
        return false
    }
    if(DATA[x] && DATA[x][y - 1] && (DATA[x][y - 1].state == LOCKED || DATA[x][y - 1].state == PROCESSING)) {
        return false
    }
    if(DATA[x] && DATA[x][y + 1] && (DATA[x][y + 1].state == LOCKED || DATA[x][y + 1].state == PROCESSING)) {
        return false
    }

    DATA[x][y].state = PROCESSING
    if(DATA[x + 1] && DATA[x + 1][y]) {
        DATA[x + 1][y].state = LOCKED
    }
    if(DATA[x - 1] && DATA[x - 1][y]) {
        DATA[x - 1][y].state = LOCKED
    }
    if(DATA[x] && DATA[x][y - 1]) {
        DATA[x][y - 1].state = LOCKED
    }
    if(DATA[x] && DATA[x][y + 1]) {
        DATA[x][y + 1].state = LOCKED
    }
    return true
}

function releaseCellLocks(x, y) {
    // DATA[tls.x][tls.x].state
    DATA[x][y].state = IDLE
    if(DATA[x + 1] && DATA[x + 1][y]) {
        if(DATA[x + 1][y].progress == 0) {
            DATA[x + 1][y].state = DONE
        } else {
            DATA[x + 1][y].state = IDLE
        }
    }
    if(DATA[x - 1] && DATA[x - 1][y]) {
        if(DATA[x - 1][y].progress == 0) {
            DATA[x - 1][y].state = DONE
        } else {
            DATA[x - 1][y].state = IDLE
        }
    }
    if(DATA[x] && DATA[x][y - 1]) {
        if(DATA[x][y - 1].progress == 0) {
            DATA[x][y - 1].state = DONE
        } else {
            DATA[x][y - 1].state = IDLE
        }
    }
    if(DATA[x] && DATA[x][y + 1]) {
        if(DATA[x][y + 1].progress == 0) {
            DATA[x][y + 1].state = DONE
        } else {
            DATA[x][y + 1].state = IDLE
        }
    }
}

var run = 0

var finished = false

function simulate() {
    if(finished) {
        return
    }
    console.log("run " + run)
    run++

    finished = true

    for (var proc = 0; proc < NUM_PROCESSORS; proc++) {
        var tls = threadLocalStorage[proc]
        if(tls.done) {
            continue
        }
        finished = false
        if(tls.state == LOOKING) {
            var found = false
            if(tls.x == MAX_WIDTH-1 || tls.y == MAX_HEIGHT-1) {
                tls.x = 0
                tls.y = 0
            }
            for (;tls.x < MAX_WIDTH && !found; tls.x++) {
                for (tls.y = 0;tls.y < MAX_HEIGHT && !found; tls.y++) {
                    if(acquireCellLocks(tls.x, tls.y)) {
                        if(DATA[tls.x][tls.y].progress > 0) {
                            found = true;
                            tls.state = PROCESSING;
                        } else {
                            releaseCellLocks(tls.x, tls.y)
                        }
                    }
                    if(found) {
                        break
                    }
                }
                if(found) {
                    break
                }
            }
            if(!found) {
                tls.done = true
                console.log("Process", proc, "stopped looking for work")
            }
        }
        if(tls.state == PROCESSING) {
            var cell = DATA[tls.x][tls.y]
            if(cell.progress > 0) {
                cell.progress--
            } else if (cell.progress == 0) {
                releaseCellLocks(tls.x, tls.y)
                cell.state = DONE
                tls.state = LOOKING
            }
        }
    }
    regularTable.draw()
}

regularTable.setDataListener(dataListener);
regularTable.addStyleListener(styleListener);
regularTable.draw()

 setInterval(simulate, 1000);
