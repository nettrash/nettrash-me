using System;
using System.Globalization;
using System.Web;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace nettrash.ru.Controllers
{
	[ApiController]
	[Route("[controller]")]
	public class TimeController : ControllerBase
	{
		#region Private properties



		private readonly ILogger<TimeController> _logger;



		#endregion
		#region Public constructors



		public TimeController(ILogger<TimeController> logger)
		{
			_logger = logger;
		}



		#endregion
		#region Public methods



		[HttpGet("unixtime")]
		public Response.Unixtime Unixtime(string source)
		{
			string src = HttpUtility.UrlDecode(source);
			try
			{
				string sResult = "";
				if (long.TryParse(src, out long unixtime))
				{
					sResult = DateTimeOffset.FromUnixTimeSeconds(unixtime).ToString("yyyy-MM-dd HH:mm:ss zzz");
				}
				else
				{
					sResult = DateTimeOffset.ParseExact(src, "yyyy-MM-dd HH:mm:ss zzz", CultureInfo.InvariantCulture).ToUnixTimeSeconds().ToString();
				}
				return new Response.Unixtime { Result = true, Value = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Unixtime { Result = false, Value = ex.Message };
			}
		}



		#endregion
	}
}