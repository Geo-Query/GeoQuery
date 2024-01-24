export const copyFile = async (sourceHandle, destinationDirectoryHandle) => {
    try {
      const newFileHandle = await destinationDirectoryHandle.getFileHandle(
        sourceHandle.name,
        { create: true }
      );
      const writable = await newFileHandle.createWritable();
      const fileData = await sourceHandle.getFile();
      await writable.write(fileData);
      await writable.close();
    } catch (error) {
      console.error('Error copying file:', error);
    }
  };
  