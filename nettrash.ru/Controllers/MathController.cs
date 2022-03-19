using System;
using System.Linq;
using System.Security.Cryptography;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace nettrash.ru.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class MathController : ControllerBase
    {

        private readonly ILogger<MathController> _logger;

        public MathController(ILogger<MathController> logger)
        {
            _logger = logger;
        }

        [HttpGet("guid")]
        public Response.Guid Guid()
        {
            return new Response.Guid { Value = System.Guid.NewGuid().ToString() };
        }

		[HttpGet("luhn")]
		public Response.Luhn Luhn(string source)
        {
			try
			{
				bool bResult = false;
				string sResult = string.Empty;
				if (source.Trim().All(char.IsDigit))
				{
					byte[] data = source.Trim().Select(c => byte.Parse(c.ToString())).ToArray();
					int value = 0;
					for (int i = 0; i < data.Length; i++)
					{
						if (i % 2 == 0)
						{
							int p = data[i] * 2;
							if (p > 9) p -= 9;
							value += p;
						}
						else
						{
							value += data[i];
						}
					}
					bResult = value % 10 == 0;
					sResult = bResult ? "valid" : "not valid";
				}
				else
				{
					sResult = "it's not a number";
				}
				return new Response.Luhn { result = true, luhnResult = bResult, errorText = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Luhn { result = false, luhnResult = false, errorText = ex.Message };
			}
		}

		[HttpPost("hash")]
		public Response.Hash Hash(Request.Hash request)
        {
			try
			{
				HashAlgorithm hash = HashAlgorithm.Create(request.algorithm);
				byte[] hashValue = hash.ComputeHash(System.Text.Encoding.UTF8.GetBytes(request.sourceText));
				string sResult = string.Join("", hashValue.Select(b => b.ToString("X2")));
				return new Response.Hash { result = true, value = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Hash { result = false, value = ex.Message };
			}

		}
	}
}
