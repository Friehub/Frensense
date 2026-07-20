// SAFE: The glob pattern is scoped to public assets only, and the file list is filtered to safe filenames

'use client'

const publicImages = import.meta.glob('/public/images/*.{png,jpg,webp}')

export default function ImageGallery() {
  const entries = Object.entries(publicImages) as [string, () => Promise<unknown>][]

  return (
    <div>
      {entries.map(([path]) => {
        const filename = path.split('/').pop() ?? ''
        return <img key={path} src={`/images/${filename}`} alt={filename} />
      })}
    </div>
  )
}
