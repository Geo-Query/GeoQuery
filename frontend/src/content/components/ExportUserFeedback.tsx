// //UserFeedback.tsx
// import React from 'react';
// import { ExportState } from './ExportWizardState';

// interface UserFeedbackProps {
//   status: ExportState;
// }

// const UserFeedback: React.FC<UserFeedbackProps> = ({ status }) => {
//     console.log("UserFeedback status:", status); // Log the status for debugging  
//     const { isLoading, isSuccess, message } = status;

//   if (isLoading) {
//     // This could be a spinner or some loading indicator
//     return <div>Loading...</div>;
//   }

//   if (message) {
//     const feedbackClass = isSuccess ? 'text-green-700 bg-green-200' : 'text-red-700 bg-red-200';
//     return (
//       <div className={`p-2 rounded-lg ${feedbackClass}`}>
//         {message}
//       </div>
//     );
//   }

//   // If there is no message, don't render anything
//   return null;
// };

// export default UserFeedback;

//! deprecated due to IPC implementation - resuse as needed.
