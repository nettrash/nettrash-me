using System;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace nettrash.Controllers
{
	[ApiController]
	[Route("[controller]")]
	public class HomeController : ControllerBase
	{
		#region Private properties



		private readonly ILogger<TextController> _logger;



		#endregion
		#region Public constructors



		public HomeController(ILogger<TextController> logger)
		{
			_logger = logger;
		}



		#endregion
		#region Public methods



		[HttpPost("info")]
		public Response.ClientInfo ClientIp()
		{
			try
			{
				string ip = HttpContext.Connection.RemoteIpAddress.ToString();
				if (string.IsNullOrWhiteSpace(ip))
				{
					ip = HttpContext.Request.Headers["X-Real-IP"];
				}
				return new Response.ClientInfo { Result = true, IPAddress = HttpContext.Connection.RemoteIpAddress.ToString() };
			}
			catch (Exception ex)
			{
				return new Response.ClientInfo { Result = false, IPAddress = ex.Message };
			}
		}



		#endregion
	}
}