import { FileImage, FileText } from "lucide-react";

const downloads = [
  {
    label: "Light PDF",
    icon: FileImage,
    filename: "/hmziqrs-light-cv.pdf",
  },
  {
    label: "Dark PDF",
    icon: FileImage,
    filename: "/hmziqrs-dark-cv.pdf",
  },
  {
    label: "Light JPEG",
    icon: FileText,
    filename: "/hmziqrs-light-cv.jpg",
  },
  {
    label: "Dark JPEG",
    icon: FileText,
    filename: "/hmziqrs-dark-cv.jpg",
  },
] as const;

export function DownloadCV() {
  return (
    <div className="flex bg-zinc-100 dark:bg-zinc-900 print:hidden jpeg">
      <div className="container max-w-8xl mx-auto px-6 py-10">
        <h2 className="text-2xl font-semibold text-zinc-800 dark:text-zinc-100 mb-8">
          Download CV
        </h2>

        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
          {downloads.map((download) => (
            <a
              key={download.filename}
              href={download.filename}
              download
              className="flex items-center justify-center gap-3 px-6 py-4
                bg-zinc-200 dark:bg-zinc-800
                hover:bg-zinc-300 dark:hover:bg-zinc-700
                text-zinc-800 dark:text-zinc-200
                rounded-md transition-colors duration-300"
            >
              <download.icon size={20} />
              <span className="font-medium">{download.label}</span>
            </a>
          ))}
        </div>
      </div>
    </div>
  );
}