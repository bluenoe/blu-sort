
import React, { useMemo, useState } from 'react'
if (allSelected) setSelected(new Set())
else setSelected(new Set(items.map((_,i)=>i)))
}


return (
<div className="p-6 max-w-6xl mx-auto space-y-6">
<header className="flex items-center justify-between">
<h1 className="text-2xl font-bold">BluSorter</h1>
<div className="flex gap-2">
<button className="btn" onClick={undo}>Undo</button>
<a className="btn" href="https://github.com/" target="_blank">Help</a>
</div>
</header>


<section className="card p-4 grid grid-cols-1 md:grid-cols-3 gap-3">
<div className="md:col-span-2 flex items-center gap-3">
<input className="input" placeholder="Paste folder path (e.g. E:\\Downloads)" value={dir} onChange={e=>setDir(e.target.value)} />
<button className="btn" onClick={scan} disabled={loading || !dir}>{loading? 'Scanning…' : 'Dry‑run'}</button>
</div>
<div className="flex items-center gap-3">
<button className="btn w-full" onClick={apply} disabled={loading || selected.size===0}>Apply</button>
</div>
<div className="md:col-span-3 text-sm text-zinc-400">
{summary.total>0 && (
<span>Preview: {summary.total} items • Duplicates: {summary.duplicates} • Selected: {selected.size}</span>
)}
{message && <span className="ml-3 text-zinc-200">{message}</span>}
</div>
</section>


<section className="card p-0 overflow-hidden">
<div className="overflow-auto">
<table className="w-full text-sm">
<thead className="bg-zinc-900 sticky top-0">
<tr>
<th className="text-left p-3 w-10"><input type="checkbox" checked={allSelected} onChange={toggleAll} /></th>
<th className="text-left p-3">From</th>
<th className="text-left p-3">→</th>
<th className="text-left p-3">To</th>
<th className="text-left p-3">Reason</th>
<th className="text-left p-3">Dup?</th>
</tr>
</thead>
<tbody>
{items.map((it, i)=> (
<tr key={i} className="odd:bg-zinc-950 even:bg-zinc-900/40">
<td className="p-3"><input type="checkbox" checked={selected.has(i)} onChange={()=>toggle(i)} /></td>
<td className="p-3 font-mono text-xs break-all">{it.from}</td>
<td className="p-3">→</td>
<td className="p-3 font-mono text-xs break-all">{it.to}</td>
<td className="p-3">{it.reason}</td>
<td className="p-3">{it.duplicate ? 'Yes' : ''}</td>
</tr>
))}
</tbody>
</table>
</div>
</section>
</div>
)
}