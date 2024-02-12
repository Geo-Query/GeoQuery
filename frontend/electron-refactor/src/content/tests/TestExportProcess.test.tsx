jest.mock('fs', () => ({
    promises: {
      readFile: jest.fn().mockImplementation(() => Promise.resolve(JSON.stringify({ /* Mock Data */ }))),
      mkdir: jest.fn().mockResolvedValue(undefined),
      copyFile: jest.fn().mockResolvedValue(undefined),
    },
  }));