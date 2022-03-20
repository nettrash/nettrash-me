using System.Diagnostics;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.Extensions.Logging;

namespace nettrash.ru.Pages
{
	[ResponseCache(Duration = 0, Location = ResponseCacheLocation.None, NoStore = true)]
	public class ErrorModel : PageModel
	{
		#region Private properties



		private readonly ILogger<ErrorModel> _logger;



		#endregion
		#region Public properties



		public string RequestId { get; set; }

		public bool ShowRequestId => !string.IsNullOrEmpty(RequestId);



		#endregion
		#region Public constructors



		public ErrorModel(ILogger<ErrorModel> logger)
		{
			_logger = logger;
		}



		#endregion
		#region Public methods



		public void OnGet()
		{
			RequestId = Activity.Current?.Id ?? HttpContext.TraceIdentifier;
		}



		#endregion
	}
}