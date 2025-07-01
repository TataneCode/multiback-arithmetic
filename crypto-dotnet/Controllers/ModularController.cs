using Congruent;
using Grpc.Net.Client;
using Microsoft.AspNetCore.Mvc;

namespace crypto_dotnet.Controllers;

[ApiController]
[Route("[controller]")]
public class ModularController : ControllerBase
{
  [HttpGet("congruent")]
  public async Task<IActionResult> GetCongruent([FromQuery] int n1, [FromQuery] int n2, [FromQuery] int modulo)
  {
    // Crée le canal vers le service Rust (adapter l’URL/port si besoin)
    using var channel = GrpcChannel.ForAddress("http://rust-api:50051");
    var client = new ModularService.ModularServiceClient(channel);

    var request = new CongruentRequest { N1 = n1, N2 = n2, Modulo = modulo };
    var response = await client.IsCongruentAsync(request);

    return Ok(new
    {
      n1_remaining = response.N1Remaining,
      n2_remaining = response.N2Remaining,
      is_congruent = response.IsCongruent
    });
  }
}